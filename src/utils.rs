use std::{ops::Range, str::Chars};

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

pub struct WeekRange<'a> {
    input: Chars<'a>,

    range: Option<Range<u8>>,
}

impl<'a> From<&'a str> for WeekRange<'a> {
    fn from(value: &'a str) -> Self {
        WeekRange {
            input: value.chars(),
            range: None,
        }
    }
}

impl Iterator for WeekRange<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(range) = self.range.as_mut() {
            if let Some(next) = range.next() {
                return Some(next);
            } else {
                self.range = None;
            }
        }

        let mut temp = String::new();

        let mut start = 0;

        let iter = self.input.by_ref();

        while let Some(c) = iter.next() {
            println!("{}", c);
            if c == '-' {
                start = temp.parse().ok()?;
                temp.clear();
                continue;
            }

            if c == ',' && iter.next() == Some(' ') {
                   break;
            }

            temp.push(c);
        }

        let end: u8 = temp.parse().ok()?;

        if start == 0 {
            return Some(end);
        }

        self.range = Some(start + 1..end + 1);

        Some(start)
    }
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
mod url_macros {
    #[test]
    fn rest_example() {
        let base = "https://example.com";
        assert_eq!(rest!(base, test), "https://example.com/rest/app/test");
    }
}

#[cfg(test)]
mod parsers {
    use crate::utils;

    #[test]
    fn date() {
        let date = "2021-01-01";
        let parsed = utils::parse_date(date).unwrap();
        assert_eq!(parsed, chrono::NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
    }

    #[test]
    fn datetime() {
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

#[cfg(test)]
mod week_parser {
    use super::WeekRange;

    #[test]
    fn single() {
        let input = "11";
        let expected: Vec<u8> = vec![11];
        let actual: Vec<u8> = WeekRange::from(input).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn range() {
        let input = "13-17";
        let expected: Vec<u8> = vec![13, 14, 15, 16, 17];
        let actual: Vec<u8> = WeekRange::from(input).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn complex() {
        let input = "30-37, 39, 40-42, 44-50";
        let expected: Vec<u8> = vec![30, 31, 32, 33, 34, 35, 36, 37, 39, 40, 41, 42, 44, 45, 46, 47, 48, 49, 50];
        let actual: Vec<u8> = WeekRange::from(input).collect();

        assert_eq!(expected, actual);
    }
}
