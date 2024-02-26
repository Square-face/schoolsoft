use std::str::FromStr;

use crate::types::error::RequestError;
use reqwest::StatusCode;
use serde::de::Error;

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
    chrono::NaiveDate::from_str(raw)
}

pub fn parse_datetime(raw: &str) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
    chrono::NaiveDateTime::from_str(raw)
}

#[macro_export]
macro_rules! url {
    ($base:expr, $path:ident) => {
        format!("{}/rest/app/{}", $base, stringify!($path))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro() {
        let base = "https://example.com";
        assert_eq!(url!(base, test), "https://example.com/rest/app/test");
    }
}
