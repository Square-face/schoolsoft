use mockito::Server;
use schoolsoft::{deserializers::Deserializer, schedule::Schedule};

use crate::mock::{basic_user_with_token, get};

mod mock;

/// Test deserializing the entire schedule from a json file containing a response that came from
/// schoolsofts api
#[test]
fn from_example() {
    Schedule::deserialize(include_str!("../hurl/output/schedule.json"))
        .expect("Deserializing entire schedule should work");
}

/// Test the full flow of getting the schedule
#[tokio::test]
async fn request() {
    let mut server = Server::new();

    let mock = get(
        &mut server,
        "api/lessons/student/1",
        include_str!("../hurl/output/schedule.json"),
        None,
    );

    let mut user = basic_user_with_token(&server.url());

    let res = user.get_schedule().await;

    mock.assert();

    res.expect("Getting schedule should be successful");
}
