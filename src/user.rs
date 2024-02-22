use chrono::Duration;
use serde::de::Error;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::{errors::TokenError, url};

#[derive(Debug, Clone, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum UserType {
    Student = 1,
    Parent = 2,
    Teacher = 3,
}
impl UserType {
    fn from_u8(user_type: u8) -> Option<UserType> {
        match user_type {
            1 => Some(UserType::Student),
            2 => Some(UserType::Parent),
            3 => Some(UserType::Teacher),
            _ => None,
        }
    }
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
    now: fn() -> chrono::NaiveDateTime,

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
#[derive(Debug, Clone)]
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
    school_url: String,
    client: reqwest::Client,

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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct RawOrg {
    pub name: String,
    pub blogger: bool,
    pub school_type: u32,
    pub leisure_school: u32,
    pub class: String,
    pub org_id: u32,
    pub token_login: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct RawUser {
    pub picture_url: String,
    pub name: String,
    pub is_of_age: bool,
    pub app_key: String,
    pub orgs: Vec<RawOrg>,
    #[serde(rename = "type")]
    pub user_type: u8,
    pub user_id: u32,
}

impl User {
    pub fn new(
        school_url: String,
        name: String,
        app_key: String,
        user_type: UserType,
        id: u32,
        orgs: Vec<Org>,
    ) -> Self {
        Self {
            school_url,
            client: reqwest::Client::new(),
            name,
            pictute_url: String::new(),
            is_of_age: false,
            app_key,
            token: None,
            user_type,
            id,
            orgs,
        }
    }

    /// Deserialize a user from a JSON String
    ///
    /// # Arguments
    /// * `json` - A JSON string containing the user data
    /// * `school_url` - The url of the school the user is logging in to
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::user::User;
    /// let user = User::deserialize(r#"{
    ///     "pictureUrl": "pictureFile.jsp?studentId=1337",
    ///     "name": "Mock User",
    ///     "isOfAge": false,
    ///     "appKey": "123notreal",
    ///     "orgs": [
    ///      {
    ///          "name": "Mock School",
    ///          "blogger": false,
    ///          "schoolType": 9,
    ///          "leisureSchool": 0,
    ///          "class": "F35b",
    ///          "orgId": 1,
    ///          "tokenLogin": "https://sms1.schoolsoft.se/mock_school/jsp/app/TokenLogin.jsp?token=TOKEN_PLACEHOLDER&orgid=1&childid=1337&redirect=https%3A%2F%2Fsms1.schoolsoft.se%2mock_school%2Fjsp%2Fstudent%2Fright_student_startpage.jsp"
    ///      }
    ///      ],
    ///      "type": 1,
    ///      "userId": 1337
    /// }"#,
    /// "https://example.com/mock_school".to_string(),
    /// ).expect("Failed to deserialize JSON");
    /// ```
    pub fn deserialize(json: &str, school_url: String) -> Result<User, serde_json::Error> {
        let raw: RawUser = serde_json::from_str(json)?;

        let orgs = raw
            .orgs
            .into_iter()
            .map(|raw_org| Org {
                id: raw_org.org_id,
                name: raw_org.name,
                blogger: raw_org.blogger,
                school_type: raw_org.school_type,
                leisure_school: raw_org.leisure_school,
                class: raw_org.class,
                token_login: raw_org.token_login,
            })
            .collect();

        Ok(User {
            school_url,
            client: reqwest::Client::new(),
            name: raw.name,
            pictute_url: raw.picture_url,
            is_of_age: raw.is_of_age,
            app_key: raw.app_key,
            token: None,
            user_type: UserType::from_u8(raw.user_type)
                .ok_or(serde_json::Error::custom("Invalid user type"))?,
            id: raw.user_id,
            orgs,
        })
    }

    /// Get a new token for the user
    ///
    /// This method uses the app key to get a new token from the schoolsoft api. The token is then
    /// used to authenticate to the api when making other requests.
    pub async fn get_token(&mut self) -> Result<String, TokenError> {
        let request = self
            .client
            .post(url!(self.school_url, token))
            .header("appKey", &self.app_key)
            .header("deviceid", "TempleOs");

        let response = crate::utils::make_request(request)
            .await
            .map_err(TokenError::RequestError)?;

        let token = Token::deserialize(&response).map_err(TokenError::ParseError)?;

        self.token = Some(token.clone());

        Ok(token.token)
    }

    /// Get a token for the user
    ///
    /// This method returns the saved token if it is still valid, otherwise it gets a new token
    pub async fn smart_token(&mut self) -> Result<String, TokenError> {
        match &self.token {
            Some(token) if token.is_safe() => Ok(token.token.clone()),
            _ => self.get_token().await,
        }
    }
}

impl Token {
    /// Create a new token with a custom now function
    ///
    /// This is useful for testing as it allows you to set the current time to a fixed value
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::user::Token;
    /// let token = Token::new_with_now(
    ///    "notrealtoken123_1337_1".to_string(),
    ///    chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    ///    || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(11, 0, 0).unwrap(),
    /// );
    /// ```
    ///
    pub fn new_with_now(
        token: String,
        expires: chrono::NaiveDateTime,
        now: fn() -> chrono::NaiveDateTime,
    ) -> Token {
        Token {
            now,
            token,
            expires,
        }
    }

    /// Deserialize a token from a JSON string
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::user::Token;
    /// let token = Token::deserialize(
    ///   r#"{
    ///    "expiryDate":"2024-01-01 12:00:00",
    ///    "token":"notrealtoken123_1337_1"
    ///    }"#,
    /// ).expect("Failed to deserialize JSON");
    ///
    /// assert_eq!(token.token, "notrealtoken123_1337_1".to_string());
    /// assert_eq!(token.expires, chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap());
    /// ```
    pub fn deserialize(json: &str) -> Result<Token, serde_json::Error> {
        #[derive(Deserialize)]
        struct RawToken {
            #[serde(rename = "expiryDate")]
            expiry_date: String,
            token: String,
        }

        let raw_token: RawToken = serde_json::from_str(json)?;

        Ok(Token {
            now: || chrono::Utc::now().naive_utc(),
            token: raw_token.token,
            expires: chrono::NaiveDateTime::parse_from_str(
                &raw_token.expiry_date,
                "%Y-%m-%d %H:%M:%S%.f",
            )
            .map_err(serde_json::Error::custom)?,
        })
    }

    /// Returns the duration until the token expires
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::user::Token;
    /// # use chrono::Duration;
    /// let token = Token::new_with_now(
    ///     "notrealtoken123_1337_1".to_string(),
    ///     chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    ///     || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(11, 0, 0).unwrap(),
    ///);
    ///
    /// assert_eq!(token.expires_in(), Duration::hours(1));
    /// ```
    ///
    pub fn expires_in(&self) -> Duration {
        self.expires - (self.now)()
    }

    /// Check if the token has expired
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::user::Token;
    /// let token = Token::new_with_now(
    ///    "notrealtoken123_1337_1".to_string(),
    ///    chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    ///    || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 1).unwrap(),
    /// );
    /// assert_eq!(token.is_expired(), true);
    /// ```
    ///
    pub fn is_expired(&self) -> bool {
        self.expires < (self.now)()
    }

    /// Check if the token is still valid
    ///
    /// Same as inverting [Token::is_expired()]
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::user::Token;
    /// let token = Token::new_with_now(
    ///   "notrealtoken123_1337_1".to_string(),
    ///   chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    ///   || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(11, 0, 0).unwrap(),
    /// );
    /// assert_eq!(token.is_valid(), true);
    /// ```
    ///
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    /// Check if the token is safe to use
    ///
    /// This method returns true if the token has more than 1 minute left until it expires
    ///
    /// # Example
    /// Token with more than 1 minute left
    /// ```
    /// # use schoolsoft::user::Token;
    /// let token = Token::new_with_now(
    ///  "notrealtoken123_1337_1".to_string(),
    ///  chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    ///  || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(11, 0, 0).unwrap(),
    /// );
    /// assert!(token.is_safe());
    /// ```
    /// Token with 30 sec left will return false
    /// ```
    /// # use schoolsoft::user::Token;
    /// let token = Token::new_with_now(
    ///  "notrealtoken123_1337_1".to_string(),
    ///  chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    ///  || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(11, 59, 30).unwrap(),
    /// );
    /// assert_eq!(token.is_safe(), false);
    /// ```
    pub fn is_safe(&self) -> bool {
        self.expires_in() > Duration::minutes(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod user {
        use super::*;

        #[test]
        fn deserialize_valid_json() {
            let json_data = r#"{
                "pictureUrl": "pictureFile.jsp?studentId=1337",
                "name": "Mock User",
                "isOfAge": false,
                "appKey": "123notreal",
                "orgs": [
                    {
                        "name": "Mock School",
                        "blogger": false,
                        "schoolType": 9,
                        "leisureSchool": 0,
                        "class": "F35b",
                        "orgId": 1,
                        "tokenLogin": "https://sms1.schoolsoft.se/mock_school/jsp/app/TokenLogin.jsp?token=TOKEN_PLACEHOLDER&orgid=1&childid=1337&redirect=https%3A%2F%2Fsms1.schoolsoft.se%2mock_school%2Fjsp%2Fstudent%2Fright_student_startpage.jsp"
                    }
                ],
                "type": 1,
                "userId": 1337
            }"#;

            let user = User::deserialize(json_data, "https://example.com/mock_school".to_string())
                .expect("Failed to deserialize JSON");

            assert_eq!(user.name, "Mock User");
            assert_eq!(
                user.pictute_url,
                "pictureFile.jsp?studentId=1337".to_string()
            );
            assert_eq!(user.is_of_age, false);
            assert_eq!(user.app_key, "123notreal".to_string());
            assert_eq!(user.token, None);
            assert_eq!(user.user_type, UserType::Student);
            assert_eq!(user.id, 1337);
            assert_eq!(user.orgs.len(), 1);

            let org = &user.orgs[0];
            assert_eq!(org.id, 1);
            assert_eq!(org.name, "Mock School");
            assert_eq!(org.blogger, false);
            assert_eq!(org.school_type, 9);
            assert_eq!(org.leisure_school, 0);
            assert_eq!(org.class, "F35b");
            assert_eq!(
                org.token_login,
                "https://sms1.schoolsoft.se/mock_school/jsp/app/TokenLogin.jsp?token=TOKEN_PLACEHOLDER&orgid=1&childid=1337&redirect=https%3A%2F%2Fsms1.schoolsoft.se%2mock_school%2Fjsp%2Fstudent%2Fright_student_startpage.jsp".to_string()
            );
        }
    }

    mod token {
        use super::*;

        #[test]
        fn valid_triple_decimal() {
            let token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-01-01 12:00:00.123",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON with 3 decimal places");

            assert_eq!(token.token, "123notrealtoken123_1337_1".to_string());
            assert_eq!(
                token.expires,
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
                    + chrono::Duration::milliseconds(123)
            );
        }

        #[test]
        fn valid_double_decimal() {
            let token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-02-06 21:37:15.15",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON with 2 decimal places");

            assert_eq!(token.token, "123notrealtoken123_1337_1".to_string());
            assert_eq!(
                token.expires,
                chrono::NaiveDate::from_ymd_opt(2024, 2, 6)
                    .unwrap()
                    .and_hms_opt(21, 37, 15)
                    .unwrap()
                    + chrono::Duration::milliseconds(150)
            );
        }

        #[test]
        fn valid_single_decimal() {
            let token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-02-06 21:37:15.1",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON with 1 decimal place");

            assert_eq!(token.token, "123notrealtoken123_1337_1".to_string());
            assert_eq!(
                token.expires,
                chrono::NaiveDate::from_ymd_opt(2024, 2, 6)
                    .unwrap()
                    .and_hms_opt(21, 37, 15)
                    .unwrap()
                    + chrono::Duration::milliseconds(100)
            );
        }

        #[test]
        fn valid_no_decimal() {
            let token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-02-06 21:37:15",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON with no decimal places");

            assert_eq!(token.token, "123notrealtoken123_1337_1".to_string());
            assert_eq!(
                token.expires,
                chrono::NaiveDate::from_ymd_opt(2024, 2, 6)
                    .unwrap()
                    .and_hms_opt(21, 37, 15)
                    .unwrap()
            );
        }

        #[test]
        fn expiration_in_1h() {
            let mut token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-01-01 12:00:00",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON");

            token.now = || {
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1)
                    .unwrap()
                    .and_hms_opt(11, 0, 0)
                    .unwrap()
            };

            assert_eq!(token.expires_in(), chrono::Duration::hours(1));
        }

        #[test]
        fn expiration_in_1m() {
            let mut token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-01-01 12:00:00",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON");

            token.now = || {
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1)
                    .unwrap()
                    .and_hms_opt(11, 59, 0)
                    .unwrap()
            };

            assert_eq!(token.expires_in(), chrono::Duration::minutes(1));
            assert_eq!(token.is_expired(), false);
            assert_eq!(token.is_valid(), true);
        }

        #[test]
        fn expiration_now() {
            let mut token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-01-01 12:00:00",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON");

            token.now = || {
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            };

            assert_eq!(token.expires_in(), chrono::Duration::zero());
            assert_eq!(token.is_expired(), false);
            assert_eq!(token.is_valid(), true);
        }

        #[test]
        fn expiration_expired() {
            let mut token = Token::deserialize(
                r#"{
                    "expiryDate":"2024-01-01 12:00:00",
                    "token":"123notrealtoken123_1337_1"
                }"#,
            )
            .expect("Failed to deserialize JSON");

            token.now = || {
                chrono::NaiveDate::from_ymd_opt(2024, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 1)
                    .unwrap()
            };

            assert_eq!(token.expires_in(), chrono::Duration::seconds(-1));
            assert_eq!(token.is_expired(), true);
            assert_eq!(token.is_valid(), false);
        }
    }
}
