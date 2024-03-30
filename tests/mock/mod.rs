#![allow(dead_code)]

use mockito::{Mock, ServerGuard};
use reqwest::Url;
use schoolsoft::user::{User, UserType};

pub fn basic_user(url: &str) -> User {
    User::new(
        Url::parse(url).unwrap().join("/mock_school").unwrap(),
        "mock_user".to_string(),
        "123notreal".to_string(),
        UserType::Student,
        1337,
        vec![],
    )
}

pub fn login_mock(server: &mut ServerGuard, school: &str) -> Mock {
    let url = server.url();
    server
        .mock("POST", format!("/{}/rest/app/login", school).as_str())
        .with_status(200)
        .with_body(format!(
            r#"{{
                "pictureUrl": "pictureFile.jsp?studentId=1337",
                "name": "Mock User",
                "isOfAge": false,
                "appKey": "123notreal",
                "orgs": [
                    {{
                        "name": "Mock School",
                        "blogger": false,
                        "schoolType": 9,
                        "leisureSchool": 0,
                        "class": "F35b",
                        "orgId": 1,
                        "tokenLogin": "{url}/{school}/jsp/app/TokenLogin.jsp?token=TOKEN_PLACEHOLDER&orgid=1&childid=1337&redirect=https%3A%2F%2Fsms1.schoolsoft.se%2mock_school%2Fjsp%2Fstudent%2Fright_student_startpage.jsp"
                    }}
                ],
                "type": 1,
                "userId": 1337
            }}"#),
        )
        .create()
}

pub fn token_mock(server: &mut ServerGuard, token: &str, expiration: Option<&str>) -> Mock {
    let expi_date = expiration.unwrap_or("2024-02-12 17:22:23.714");
    server
        .mock("POST", "/mock_school/rest/app/token")
        .with_status(200)
        .with_body(format!(
            r#"{{
                "expiryDate":"{expi_date}",
                "token":"{token}"
            }}"#
        ))
        .create()
}

/// Create a mock server with a login, token and primary mock with a GET route
pub fn get_with_token_and_login(server: &mut ServerGuard, path:&str, body: &str) -> (Mock, Mock, Mock) {
    let login = login_mock(server, "mock_school");

    let token = token_mock(server, "one_of_those_tokens", None);

    let path = format!("/mock_school/{}", path);

    let primary = server
        .mock("GET", path.as_str())
        .match_header("token", "one_of_those_tokens")
        .with_status(200)
        .with_body(body)
        .create();

    (login, token, primary)
}
