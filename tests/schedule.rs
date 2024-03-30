use mock::get_with_token_and_login;
use mockito::Server;
use schoolsoft::{deserializers::Deserializer, schedule::Schedule, ClientBuilder};

mod mock;

/// Test deserializing the entire schedule from a json file containing a response that came from
/// schoolsofts api
#[test]
fn from_example() {
    let data = include_str!("../hurl/output/schedule.json");

    Schedule::deserialize(data).expect("Deserializing entire schedule should work");
}

/// Test the full flow of getting the schedule
#[tokio::test]
async fn full() {
    let mut server = Server::new();

    let (login, token, schedule) = get_with_token_and_login(
        &mut server,
        "api/lessons/student/1",
        include_str!("../hurl/output/schedule.json"),
    );

    let mut client = ClientBuilder::new().base_url(server.url()).build();

    client
        .login("mock_username", "mock_password", "mock_school")
        .await
        .expect("Login should be successful");

    login.assert();

    let mut user = client.user.expect("User should be set after login");

    let res_schedule = user.get_schedule().await;

    token.assert();
    schedule.assert();

    res_schedule.expect("Getting schedule should be successful");
}
