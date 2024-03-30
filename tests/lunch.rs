use crate::mock::{basic_user_with_token, get};

mod mock;

#[tokio::test]
async fn request() {
    let mut server = mockito::Server::new();

    let mock = get(
        &mut server,
        "api/lunchmenus/student/1",
        include_str!("../hurl/output/lunch.json"),
        Some("mock_school"),
    );

    let mut user = basic_user_with_token(&server.url());

    let response = user.get_lunch().await;

    mock.assert();

    response.expect("Failed to get lunch");
}
