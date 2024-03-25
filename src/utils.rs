use crate::{
    types::error::RequestError,
    user::{User, UserType},
};
use reqwest::StatusCode;

pub fn check_codes(code: StatusCode) -> Result<(), RequestError> {
    if code.is_success() {
        return Ok(());
    }

    match code {
        StatusCode::UNAUTHORIZED => Err(RequestError::Unauthorized),
        StatusCode::INTERNAL_SERVER_ERROR => Err(RequestError::InternalServerError),
        _ => Err(RequestError::UncheckedCode(code)),
    }
}

pub async fn make_request(regeuest: reqwest::RequestBuilder) -> Result<String, RequestError> {
    let response = regeuest.send().await.map_err(RequestError::RequestError)?;
    let code = response.status();
    check_codes(code)?;

    let data = response.text().await.map_err(RequestError::ReadError)?;
    Ok(data)
}

pub fn parse_date(raw: &str) -> Result<chrono::NaiveDate, chrono::ParseError> {
    chrono::NaiveDate::parse_from_str(raw, "%Y-%m-%d")
}

pub fn parse_datetime(raw: &str) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
    chrono::NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S%.f")
}

pub fn api(user: &User, path: &str) -> String {
    format!(
        "{}/api/{}/{}/{}",
        user.school_url,
        path,
        user.user_type.to_string(),
        user.orgs[0].id
    )
}

impl ToString for UserType {
    fn to_string(&self) -> String {
        match self {
            UserType::Student => "student",
            UserType::Parent => "parent",
            UserType::Teacher => "teacher",
        }
        .to_string()
    }
}

#[macro_export]
macro_rules! rest {
    ($base:expr, $path:ident) => {
        format!("{}/rest/app/{}", $base, stringify!($path))
    };
}

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn test_macro() {
        let base = "https://example.com";
        assert_eq!(rest!(base, test), "https://example.com/rest/app/test");
    }

    #[test]
    fn test_parse_date() {
        let date = "2021-01-01";
        let parsed = utils::parse_date(date).unwrap();
        assert_eq!(parsed, chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
    }

    #[test]
    fn test_parse_datetime() {
        let date = "2021-01-01 12:00:00.000";
        let parsed = utils::parse_datetime(date).unwrap();
        assert_eq!(
            parsed,
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1)
                .unwrap()
                .and_hms_milli_opt(12, 0, 0, 0)
                .unwrap()
        );
    }
}
