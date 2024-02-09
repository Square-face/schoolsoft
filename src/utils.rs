use reqwest::StatusCode;

use crate::errors::RequestError;

pub fn check_codes<T>(code: StatusCode) -> Option<RequestError<T>> {
    if code.is_success() {
        return None;
    }

    match code {
        StatusCode::UNAUTHORIZED => Some(RequestError::InvalidCredentials),
        _ => Some(RequestError::UncheckedCode(code)),
    }
}
