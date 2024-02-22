
/// General error that can happen in most cases when making a request.
#[derive(Debug)]
pub enum RequestError {
    /// Error when sending the request.
    RequestError(reqwest::Error),

    /// Error when reading the response.
    ReadError(reqwest::Error),

    /// The given credentials are invalid.
    Unauthorized,

    /// Something went wrong on the server.
    InternalServerError,

    /// An unchecked status code was returned.
    UncheckedCode(reqwest::StatusCode),
}

///
#[derive(Debug)]
pub enum SchoolListingError {
    /// Error when sending the request.
    RequestError(RequestError),

    /// Error when reading the response.
    ParseError(serde_json::Error),

    BadUrl,
}

/// Error that can happen when trying to get a token.
#[derive(Debug)]
pub enum TokenError {
    /// Error when sending the request.
    RequestError(RequestError),

    /// Error when reading the response.
    ParseError(serde_json::Error),
}

#[derive(Debug)]
pub enum LoginError {
    /// Error when sending the request.
    RequestError(RequestError),

    /// Error when reading the response.
    ParseError(serde_json::Error),
}
