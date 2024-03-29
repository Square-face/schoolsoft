use mockito::Mock;
use schoolsoft::{
    deserializers::Deserializer,
    schedule::{self, RawOccasion, Schedule},
    user::{Org, User, UserType},
};

#[test]
fn full() {
    let data = include_str!("../hurl/output/schedule.json");

    let schedule = Schedule::deserialize(data).unwrap();
}

#[test]
fn single_occasion() {
    let data = r#"
    {
        "weeks": 2242995147496956,
        "excludingWeeks": 0,
        "creById": 0,
        "source": {},
        "externalRef": "",
        "subjectId": 236,
        "orgId": 1,
        "updDate": "2023-08-16 12:42:41.0",
        "updByType": -1,
        "excludeClass": 0,
        "startTime": "1970-01-01 08:20:00.0",
        "id": 36505,
        "includingWeeks": 0,
        "subjectName": "IDRIDO02 - Idrott och hälsa 2 - specialisering",
        "updById": 0,
        "creByType": -1,
        "creDate": "2023-08-16 12:42:41.0",
        "length": 70,
        "externalId": "34b0d97c-35ea-415f-9c7b-7f9492ef9fb4",
        "roomName": "IKSU",
        "periodWeeks": 2242995147496956,
        "includingWeeksString": "",
        "dayId": 0,
        "name": "",
        "absenceType": 1,
        "guid": "afbae58f-c35e-4480-bfd1-574fc8de5572",
        "excludingWeeksString": "",
        "endTime": "1970-01-01 09:30:00.0",
        "weeksString": "34-43, 45-51, 3-9, 11-13, 15-24",
        "tmpLesson": 0
    }
        "#;

    let raw: RawOccasion = serde_json::from_str(data).unwrap();

    let occasion = schedule::Occasion::try_from(raw).unwrap();
}

#[tokio::test]
async fn success() {
    let mut server = mockito::Server::new();

    let url = server.url();

    let token_mock = server
        .mock("POST", "/mock_school/rest/app/token")
        .with_status(200)
        .with_body(
            r#"{
                "expiryDate":"2024-02-12 17:22:23.714",
                "token":"one_of_those_tokens"
            }"#,
        )
        // get a new token
        .create();

    let schedule_mock = server
        .mock("GET", "/mock_school/api/lessons/student/1")
        .match_header("token", "one_of_those_tokens")
        .with_status(200)
        .with_body(include_str!("../hurl/output/schedule.json"))
        .create();

    let mut user = User::new(
        format!("{}/mock_school", url),
        "mock_user".to_string(),
        "123notreal".to_string(),
        UserType::Student,
        1337,
        vec![Org {
            id: 1,
            name: "mock_school".to_string(),
            blogger: false,
            school_type: 1,
            leisure_school: 1,
            class: "fake_class".to_string(),
            token_login: "no".to_string(),
        }],
    );

    user.get_schedule().await.expect("Failed to get schedule");

    token_mock.assert();
    schedule_mock.assert();
}
