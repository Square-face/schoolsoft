//! Type definitions for the structs and errors that the wrapper can return

use serde_repr::Deserialize_repr;
use uuid::Uuid;

/// The type of user
///
/// This enum represents the different types of users that can be logged in to schoolsoft.
#[derive(Debug, Clone, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum UserType {
    Student = 1,
    Parent = 2,
    Teacher = 3,
}

/// A schoolsoft token
///
/// A token is used to authenticate with the schoolsoft api. It is retrieved by logging in and
/// making a request to /<school>/rest/app/token with the app key gained from the login.
///
/// While a appkey never changes, a token is only valid for 3 hours after which it must be
/// refreshed using another call to /<school>/rest/app/token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// function for getting utc::now()
    pub now: fn() -> chrono::NaiveDateTime,

    /// The token itself
    pub token: String,

    /// When the token expires
    pub expires: chrono::NaiveDateTime,
}

/// A schoolsoft organization
///
/// As there is no official documentation for the schoolsoft API. It is unclear what organizations
/// even are. I assume that they are schools, but i only have one account to test with so i
/// can't be sure.
///
/// All i know is that when logging in, the api responds with a list of organizations. But so far
/// that list has only ever contained one singular organization with the same name as the school im
/// attending.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Org {
    /// Unique identifier for the organization
    pub id: u32,

    /// Human readable name of the organization
    pub name: String,

    /// Unknown
    pub blogger: bool,

    /// Unknown
    pub school_type: u32,

    /// Unknown, also, why is it a number?
    pub leisure_school: u32,

    /// If we assume that this is a school, then this is the class that the user is attending
    /// But what about teachers and parents? What does this field mean for them?
    pub class: String,

    /// Url to login to the organization using a web browser
    /// Once again, this field makes no since as you get it by logging in, so why would you need to
    /// log in again?
    /// And no its not the url for getting the token, that is /<school>/rest/app/token
    pub token_login: String,
}

/// A schoolsoft user
///
/// This struct represents a user of the schoolsoft system. It is deserialized from the JSON
/// returned by the api when logging in.
#[derive(Debug, Clone)]
pub struct User {
    pub school_url: String,
    pub client: reqwest::Client,

    /// Users full name
    pub name: String,

    /// Url to the users profile picture
    pub pictute_url: String,

    /// If the user is over 18 (schoolsoft is swedish)
    pub is_of_age: bool,

    /// The app key retrieved when logging in
    pub app_key: String,

    /// Token used for interacting with api routes that require authentication
    ///
    /// This field is not populated by logging in. Instead it requires a separate request to
    /// /<school>/rest/app/token with the app key.
    pub token: Option<Token>,

    /// What type of user this is
    pub user_type: UserType,

    /// Unique identifier for the user
    pub id: u32,

    /// List of organizations that the user is a part of
    pub orgs: Vec<Org>,
}

/// The login methods available for a school
///
/// Represented as an array of numbers. If the number for a login method is present in the array,
/// its available.
///
/// For the app api to work 4 must be present.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginMethods {
    pub student: Vec<u8>,
    pub teacher: Vec<u8>,
    pub parent: Vec<u8>,
}

/// A schoolsoft school
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchoolListing {
    pub login_methods: LoginMethods,
    pub name: String,
    pub url: String,
    pub url_name: String,
}

/// Represents a specific weeks lunch menu
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LunchMenu {
    pub week: u32,
    pub created_at: chrono::NaiveDateTime,
    pub category: String,
    pub monday: Lunch,
    pub tuesday: Lunch,
    pub wednesday: Lunch,
    pub thursday: Lunch,
    pub friday: Lunch,
}

/// Represents a specific days lunch
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lunch {
    pub date: chrono::NaiveDate,
    pub food: String,
}


/// Types used when something goes wrong
pub mod error {
    use chrono::OutOfRange;
    use thiserror::Error;

    /// General error that can happen in most cases when making a request.
    #[derive(Error, Debug)]
    pub enum RequestError {
        #[error("Error when sending request: {0}")]
        RequestError(reqwest::Error),

        #[error("Error when reading the response: {0}")]
        ReadError(reqwest::Error),

        #[error("Unauthorized")]
        Unauthorized,

        #[error("Internal server error")]
        InternalServerError,

        #[error("Response returned a unexpected status code: {0}")]
        UncheckedCode(reqwest::StatusCode),
    }

    /// Error that can happen when trying to get a list of schools.
    #[derive(Error, Debug)]
    pub enum SchoolListingError {
        #[error("Error when sending request: {0}")]
        RequestError(RequestError),

        #[error("Error when reading the response: {0}")]
        ParseError(serde_json::Error),

        #[error("The url is invalid")]
        BadUrl,
    }

    /// Error that can happen when trying to get a token.
    #[derive(Error, Debug)]
    pub enum TokenError {
        #[error("Error when sending request: {0}")]
        RequestError(RequestError),

        #[error("Error when reading the response: {0}")]
        ParseError(serde_json::Error),
    }

    /// Error that can happen when trying to login.
    #[derive(Error, Debug)]
    pub enum LoginError {
        #[error("Error when sending request: {0}")]
        RequestError(RequestError),

        #[error("Error when reading the response: {0}")]
        ParseError(serde_json::Error),
    }

    /// Error that can happen when trying to get a lunch menu.
    #[derive(Error, Debug)]
    pub enum LunchMenuError {
        #[error("Error when sending request: {0}")]
        RequestError(RequestError),

        #[error("Error when retrieving new token: {0}")]
        TokenError(TokenError),

        #[error("Error when reading the response: {0}")]
        ParseError(LunchMenuParseError),
    }

    #[derive(Error, Debug)]
    pub enum ScheduleError {
        #[error("Error when sending request: {0}")]
        RequestError(RequestError),

        #[error("Error when retrieving new token: {0}")]
        TokenError(TokenError),

        #[error("Error when reading the response: {0}")]
        ParseError(ScheduleParseError),
    }

    /// Error that can happen when trying to parse a lunch menu.
    #[derive(Error, Debug)]
    pub enum LunchMenuParseError {
        #[error("No lunch menu available")]
        NoLunchMenu,

        #[error("Error when parsing json: {0}")]
        SerdeError(serde_json::Error),

        #[error("Error when parsing date: {0}")]
        DateParseError(String, chrono::ParseError),
    }
    
    #[derive(Error, Debug)]
    pub enum ScheduleParseError {
        #[error("Error when parsing json: {0}")]
        SerdeError(serde_json::Error),

        #[error("Error when parsing date: {0}")]
        DateParseError(chrono::ParseError),

        #[error("Error when parsing time: {0}")]
        TimeParseError(chrono::ParseError),

        #[error("Error when parsing day_of_week: {0}")]
        DayOfWeekError(OutOfRange),

        #[error("Error when parsing uuid: {0}")]
        UuidParseError(uuid::Error),
    }
}
