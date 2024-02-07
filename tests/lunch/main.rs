use schoolsoft::types::{Org, User, UserType};
use tokio::test;

#[test]
async fn token_success() {
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

    let lunch_mock = server
        .mock("GET", "/mock_school/api/lunchmenus/student/1")
        .with_status(200)
        .match_header("token", "one_of_those_tokens")
        .with_body(r#"[{
            "saturday": "",
            "week": 8,
            "updById": 112,
            "creByType": -1,
            "creDate": "2024-02-16 15:03:15.0",
            "dishCategoryName": "Lunch",
            "creById": 112,
            "thursday": "Pestobakad fisk med vitvinsås och pasta penne.\r\n\r\nVeg:\r\nGrönsaksbiffar med vitvinsås och pasta penne.",
            "dates": [
                "2024-02-19",
                "2024-02-20",
                "2024-02-21",
                "2024-02-22",
                "2024-02-23",
                "2024-02-24",
                "2024-02-25"
            ],
            "orgId": 1,
            "updDate": "2024-02-16 15:03:15.0",
            "empty": false,
            "updByType": -1,
            "sunday": "",
            "tuesday": "Het köttfärssoppa med kökets bröd.\r\n\r\nVeg:\r\nHet bön och rotfruktssoppa med kökets bröd.",
            "dish": 1,
            "wednesday": "Kyckling- och gröncurry thai med ris.\r\n\r\nVeg:\r\nBlomkål- och gröncurry thai med ris.",
            "friday": "Kryddiga korvar med potatissallad och paprikamajo.\r\n\r\nVeg:\r\nKryddig sojakorv med potatissallad och paprikamajo.",
            "id": -1,
            "monday": "Pasta med strimlat fläskkött och pepparsås.\r\n\r\nVeg:\r\nPasta med vegobitar och pepparsås."
        }]"#,)
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

    user.get_lunch().await.expect("Failed to get lunch");

    token_mock.assert();
    lunch_mock.assert();
}
