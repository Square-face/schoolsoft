use schoolsoft::{
    types::{
        error::{LoginError, RequestError},
        UserType,
    },
    ClientBuilder,
};
use tokio::test;

use crate::mock::login_mock;

mod mock;

#[test]
async fn authorized() {
    let mut server = mockito::Server::new();
    let mock = login_mock(&mut server, Some("mock_school"));

    let mut client = ClientBuilder::new().base_url(server.url()).build();
    client.login("mock_username", "mock_password", "mock_school").await.expect("Login should be successful");

    let user = client.user.expect("User should be set after login");

    // Exesive ammounts of asserting
    assert!(!user.is_of_age);

    assert_eq!(user.name, "Mock User");
    assert_eq!(user.app_key, "123notreal".to_string());
    assert_eq!(
        user.pictute_url,
        "pictureFile.jsp?studentId=1337".to_string()
    );
    assert_eq!(user.app_key, "123notreal".to_string());
    assert_eq!(user.token, None);
    assert_eq!(user.user_type, UserType::Student);
    assert_eq!(user.id, 1337);
    assert_eq!(user.orgs.len(), 1);

    let org = &user.orgs[0];
    assert!(!org.blogger);
    assert_eq!(org.id, 1);
    assert_eq!(org.name, "Mock School");
    assert_eq!(org.school_type, 9);
    assert_eq!(org.leisure_school, 0);
    assert_eq!(org.class, "F35b");

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

    let response = client.login("mock_username", "mock_password", "mock_school").await;

    mock.assert();

    match response {
        Err(LoginError::RequestError(RequestError::Unauthorized)) => (),
        _ => panic!("Should return Unauthorized error"),
    }
}
