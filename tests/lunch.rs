use schoolsoft::ClientBuilder;
use tokio::test;

use crate::mock::get_with_token_and_login;

mod mock;

#[test]
async fn full() {
    let mut server = mockito::Server::new();

    let (login, token, lunch) = get_with_token_and_login(
        &mut server,
        "api/lunchmenus/student/1",
        include_str!("../hurl/output/lunch.json"),
    );

    let mut client = ClientBuilder::new().base_url(server.url()).build();

    let login_attempt = client
        .login("mock_username", "mock_password", "mock_school")
        .await;

    login.assert();
    login_attempt.expect("Login should be successful");

    let mut user = client.user.expect("User should be set after login");

    user.get_lunch().await.expect("Failed to get lunch");

    token.assert();
    lunch.assert();
}
