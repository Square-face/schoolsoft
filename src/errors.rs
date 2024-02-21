/// Error type for requests.
///
/// This type is returned when a request fails.
#[derive(Debug)]
pub enum RequestError<T> {
    /// Error when sending the request.
    RequestError(reqwest::Error),

    /// Error when reading the response.
    ReadError(reqwest::Error),

    /// Error when parsing the response.
    ParseError(T),

    /// The given credentials are invalid.
    Unauthorized,

    /// Something went wrong on the server.
    InternalServerError,

    /// An unknown error occurred.
    UnknownError,

    /// An unchecked status code was returned.
    UncheckedCode(reqwest::StatusCode),
}

#[derive(Debug)]
pub enum SchoolParseError {
    BadUrl,
    ParseError(std::num::ParseIntError),
    InvalidJson(serde_json::Error),
}

#[derive(Debug)]
pub enum TokenError {
    InvalidJson(serde_json::Error),
    InvalidTimestamp(chrono::ParseError),
}
