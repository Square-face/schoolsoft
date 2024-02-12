use chrono::{Duration, NaiveDate};
use schoolsoft::user::User;
use tokio::test;

#[test]
async fn get_token() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let mock = server
        .mock("POST", "/mock_school/rest/app/token")
        .with_status(200)
        .with_body(
            r#"{
                "expiryDate":"2024-02-12 17:22:23.714",
                "token":"662a193b25bf2d38c75bb1b5f87d0562_3881_1"
            }"#,
        )
        .create();
    let mut user = User::deserialize(
        r#"{
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
        }"#,
        format!("{}/mock_school", url),
    ).expect("Failed to deserialize user");

    user.get_token().await.expect("Failed to get token");

    user.token.clone().expect("Token not set");
    assert_eq!(&(user.token.clone().unwrap().token), &"662a193b25bf2d38c75bb1b5f87d0562_3881_1");
    assert_eq!(&(user.token.clone().unwrap().expires), &(NaiveDate::from_ymd_opt(2024, 02, 12).unwrap().and_hms_opt(17, 22, 23).unwrap() + Duration::milliseconds(714)));

    mock.assert();
}
