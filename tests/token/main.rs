use chrono::{Duration, NaiveDate};
use schoolsoft::{
    types::error::{RequestError, TokenError},
    types::{User, UserType},
};
use tokio::test;

#[test]
async fn token_success() {
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

    let mut user = User::new(
        format!("{}/mock_school", url),
        "mock_user".to_string(),
        "123notreal".to_string(),
        UserType::Student,
        1337,
        vec![],
    );

    user.get_token().await.expect("Failed to get token");

    user.token.clone().expect("Token not set");
    assert_eq!(
        &(user.token.clone().unwrap().token),
        &"662a193b25bf2d38c75bb1b5f87d0562_3881_1"
    );
    assert_eq!(
        &(user.token.clone().unwrap().expires),
        &(NaiveDate::from_ymd_opt(2024, 2, 12)
            .unwrap()
            .and_hms_opt(17, 22, 23)
            .unwrap()
            + Duration::milliseconds(714))
    );

    mock.assert();
}

#[test]
async fn token_bad_auth() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let mock = server
        .mock("POST", "/mock_school/rest/app/token")
        .with_status(401)
        .create();

    let mut user = User::new(
        format!("{}/mock_school", url),
        "mock_user".to_string(),
        "123notreal".to_string(),
        UserType::Student,
        1337,
        vec![],
    );

    let result = user.get_token().await;

    match result {
        Err(TokenError::RequestError(RequestError::Unauthorized)) => {}
        _ => panic!("Expected Unauthorized, got {:?}", result),
    }

    mock.assert();
}
