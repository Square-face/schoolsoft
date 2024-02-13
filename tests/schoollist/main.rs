use schoolsoft::ClientBuilder;
use tokio::test;

#[test]
async fn single() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let mock = server
        .mock("GET", "/rest/app/schoollist/prod")
        .with_status(200)
        .with_body(
            r#"
            [
                {
                    "studentLoginMethods": "0,1,2,3,4",
                    "parentLoginMethods": "0",
                    "name": "Carl Wahren Gymnasium",
                    "teacherLoginMethods": "",
                    "url": "https://sms.schoolsoft.se/carlwahren/"
                }
            ]
            "#,
        )
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

    let mock = server
        .mock("GET", "/rest/app/schoollist/prod")
        .with_status(200)
        .with_body(include_str!("../../hurl/school-list.json"))
        .create();

    let client = ClientBuilder::new().base_url(url).build();

    let response = client.schools().await;

    let schools = response.unwrap();

    assert_eq!(schools.len(), 3060);

    mock.assert();
}
