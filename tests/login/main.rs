use schoolsoft::{
    types::{
        error::{LoginError, RequestError},
        UserType,
    },
    ClientBuilder,
};
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
        .with_body(r#"{ "error": "Invalid username or password" }"#)
        .create();

    let mut client = ClientBuilder::new().base_url(url).build();

    let response = client.login("mock_username", "mock_password", "mock_school");

    match response.await {
        Ok(_) => panic!("Expected error"),
        Err(LoginError::RequestError(RequestError::Unauthorized)) => (),
        Err(_) => panic!("Unexpected error"),
    }

    mock.assert();
}
