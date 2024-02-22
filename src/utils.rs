use reqwest::StatusCode;

use crate::errors::RequestError;

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
