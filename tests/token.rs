use chrono::{Duration, NaiveDate};
use schoolsoft::types::error::{RequestError, TokenError};
use tokio::test;

use crate::mock::{basic_user, token_mock};

mod mock;

#[test]
async fn token_success() {
    let mut server = mockito::Server::new();

    let url = server.url();

    #[allow(deprecated)]
    let expiration_date =
        NaiveDate::from_ymd(2024, 2, 12).and_hms(17, 22, 23) + Duration::milliseconds(714);
    let target_token = "123notreal";

    let mock = token_mock(
        &mut server,
        target_token,
        Some(&expiration_date.to_string()),
    );

    let token = basic_user(&url)
        .get_token()
        .await
        .expect("Retrieving a token should work");

    assert_eq!(token.token, target_token);
    assert_eq!(token.expires, expiration_date);

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

    let result = basic_user(&url)
        .get_token()
        .await
        .expect_err("This should not work");

    if let TokenError::RequestError(RequestError::Unauthorized) = result {
    } else {
        panic!("Expected Unauthorized, got {:?}", result)
    }

    mock.assert();
}
