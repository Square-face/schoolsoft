use std::num::ParseIntError;


/// Error type for requests.
///
/// This type is returned when a request fails.
#[derive(Debug)]
pub enum RequestError {
    /// Error when sending the request.
    RequestError(reqwest::Error),

    /// Error when reading the response.
    ReadError(reqwest::Error),

    /// Error when parsing the response.
    ParseError(serde_json::Error),

    /// The given credentials are invalid.
    InvalidCredentials,

    /// An unknown error occurred.
    UnknownError,

    /// An unchecked status code was returned.
    UncheckedCode(reqwest::StatusCode),
}


#[derive(Debug)]
pub enum SchoolError {
    RequestError(reqwest::Error),
    ReadError(reqwest::Error),
    InvalidJson(serde_json::Error),
    ParseError(ParseIntError),
    BadUrl,
    UnknownError,
    UncheckedCode(reqwest::StatusCode),
}
