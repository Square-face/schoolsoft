
#[cfg(test)]
mod login {
    use schoolsoft::{user::UserType, ClientBuilder, errors::RequestError};
    use tokio::test;

    #[test]
    async fn success() {
        let mut server = mockito::Server::new();

        let url = server.url();

        let mock = server.mock("POST", "/mock_school/rest/app/login")
            .with_status(200)
            .with_body(r#"{
                "pictureUrl": "pictureFile.jsp?studentId=1337",
                "name": "Mock User",
                "isOfAge": false,
                "appKey": "123notreal",
                "orgs": [
                    {
                        "name": "Mock School",
                        "blogger": false,
                        "schoolType": 9,
                        "leisureSchool": 0,
                        "class": "F35b",
                        "orgId": 1,
                        "tokenLogin": "https://sms1.schoolsoft.se/mock_school/jsp/app/TokenLogin.jsp?token=TOKEN_PLACEHOLDER&orgid=1&childid=1337&redirect=https%3A%2F%2Fsms1.schoolsoft.se%2mock_school%2Fjsp%2Fstudent%2Fright_student_startpage.jsp"
                    }
                ],
                "type": 1,
                "userId": 1337
            }"#)
            .create();

        let mut client = ClientBuilder::new().base_url(url).build();

        let response = client.login("mock_username", "mock_password", "mock_school");
        assert!(response.await.is_ok());

        let user = client.user.unwrap();

        assert_eq!(user.name, "Mock User");
        assert_eq!(
            user.pictute_url,
            "pictureFile.jsp?studentId=1337".to_string()
        );
        assert_eq!(user.is_of_age, false);
        assert_eq!(user.app_key, "123notreal".to_string());
        assert_eq!(user.token, None);
        assert_eq!(user.user_type, UserType::Student);
        assert_eq!(user.id, 1337);
        assert_eq!(user.orgs.len(), 1);
        let org = &user.orgs[0];
        assert_eq!(org.id, 1);
        assert_eq!(org.name, "Mock School");
        assert_eq!(org.blogger, false);
        assert_eq!(org.school_type, 9);
        assert_eq!(org.leisure_school, 0);
        assert_eq!(org.class, "F35b");
        assert_eq!(
        org.token_login,
        "https://sms1.schoolsoft.se/mock_school/jsp/app/TokenLogin.jsp?token=TOKEN_PLACEHOLDER&orgid=1&childid=1337&redirect=https%3A%2F%2Fsms1.schoolsoft.se%2mock_school%2Fjsp%2Fstudent%2Fright_student_startpage.jsp".to_string()
    );

        mock.assert();
    }

    #[test]
    async fn failure() {
        let mut server = mockito::Server::new();

        let url = server.url();

        let mock = server
            .mock("POST", "/mock_school/rest/app/login")
            .with_status(401)
            .with_body(
                r#"{ "error": "Invalid username or password" }"#,
            )
            .create();

        let mut client = ClientBuilder::new().base_url(url).build();

        let response = client.login("mock_username", "mock_password", "mock_school");

        match response.await {
            Ok(_) => panic!("Expected error"),
            Err(RequestError::InvalidCredentials) => (),
            Err(_) => panic!("Unexpected error"),
        }

        mock.assert();
    }
}

#[cfg(test)]
mod schoollist {
    use schoolsoft::ClientBuilder;
    use tokio::test;

    #[test]
    async fn single() {
        let mut server = mockito::Server::new();

        let url = server.url();

        let mock = server.mock("GET", "/rest/app/schoollist/prod")
            .with_status(200)
            .with_body(r#"
            [
                {
                    "studentLoginMethods": "0,1,2,3,4",
                    "parentLoginMethods": "0",
                    "name": "Carl Wahren Gymnasium",
                    "teacherLoginMethods": "",
                    "url": "https://sms.schoolsoft.se/carlwahren/"
                }
            ]
            "#)
            .create();

        let client = ClientBuilder::new().base_url(url).build();

        let response = client.schools().await;

        let schools = response.unwrap();
        assert_eq!(schools.len(), 1);
        assert_eq!(schools[0].name, "Carl Wahren Gymnasium");
        assert_eq!(
            schools[0].url,
            "https://sms.schoolsoft.se/carlwahren/".to_string()
        );
        let empty: Vec<u8> = Vec::new();
        assert_eq!(schools[0].login_methods.student, vec![0, 1, 2, 3, 4]);
        assert_eq!(schools[0].login_methods.parent, vec![0]);
        assert_eq!(schools[0].login_methods.teacher, empty);

        mock.assert();
    }

    #[test]
    async fn full() {
        let mut server = mockito::Server::new();

        let url = server.url();

        let mock = server.mock("GET", "/rest/app/schoollist/prod")
            .with_status(200)
            .with_body(include_str!("../hurl/school-list.json"))
            .create();

        let client = ClientBuilder::new().base_url(url).build();

        let response = client.schools().await;

        let schools = response.unwrap();

        assert_eq!(schools.len(), 3060);

        mock.assert();
    }
}
