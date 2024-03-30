#![allow(dead_code, deprecated)]

use chrono::NaiveDate;
use mockito::{Mock, ServerGuard};
use reqwest::Url;
use schoolsoft::user::{Org, Token, User, UserType};

pub fn basic_user(url: &str) -> User {
    let url = Url::parse(url).expect("url should be parsable").join("/mock_school").expect("should be able to join with mock_school");
    User::new(
        url.clone(),
        "mock_user".to_string(),
        "123notreal".to_string(),
        UserType::Student,
        1337,
        vec![Org {
            id: 1,
            name: "Mock School".to_string(),
            blogger: false,
            school_type: 1,
            leisure_school: 1,
            class: "f35b".to_string(),
            token_login: format!("{}/jsp/app/TokenLogin.jsp?token=TOKEN_PLACEHOLDER&orgid=1&childid=1337&redirect=https%3A%2F%2Fsms1.schoolsoft.se%2Fmock_school%2Fjsp%2Fstudent%2Fright_student_startpage.jsp", url),
        }],
    )
}

/// Create a basic user with a token and a modified function for getting the current time so it
/// thinks that the token is not expired
pub fn basic_user_with_token(url: &str) -> User {
    let mut user = basic_user(url);

    let token = Token {
        token: "one_of_those_tokens".to_string(),
        now: || NaiveDate::from_ymd(2024, 2, 12).and_hms(19, 22, 23),
        expires: NaiveDate::from_ymd(2024, 2, 12).and_hms(20, 22, 23),
    };

    dbg!(token.expires_in());
    assert!(token.is_safe());

    user.token = Some(token);

    user
}

pub fn get(server: &mut ServerGuard, path: &str, body: &str, school: Option<&str>) -> Mock {
    let path = format!("/{}/{}", school.unwrap_or("mock_school"), path);
    server
        .mock("GET", path.as_str())
        .with_status(200)
        .with_body(body)
        .create()
}

pub fn post(server: &mut ServerGuard, path: &str, body: &str, school: Option<&str>) -> Mock {
    let path = format!("/{}/{}", school.unwrap_or("mock_school"), path);
    server
        .mock("POST", path.as_str())
        .with_status(200)
        .with_body(body)
        .create()
}

pub fn login_mock(server: &mut ServerGuard, school: Option<&str>) -> Mock {
    let url = server.url();
    let school = school.unwrap_or("mock_school");
    post(
        server,
        "rest/app/login",
        format!(
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
            }}"#,
        )
        .as_str(),
        Some(school),
    )
}

pub fn token_mock(server: &mut ServerGuard, token: &str, expiration: Option<&str>) -> Mock {
    let expi_date = expiration.unwrap_or("2024-02-12 17:22:23.714");
    post(
        server,
        "rest/app/token",
        format!(
            r#"{{
                "expiryDate":"{expi_date}",
                "token":"{token}"
            }}"#
        )
        .as_str(),
        None,
    )
}

/// Create a mock server with a login, token and primary mock with a GET route
pub fn get_with_token_and_login(
    server: &mut ServerGuard,
    path: &str,
    body: &str,
) -> (Mock, Mock, Mock) {
    let school = Some("mock_school");
    (
        login_mock(server, school),
        token_mock(server, "one_of_those_tokens", None),
        get(server, path, body, school),
    )
}
