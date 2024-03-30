use crate::deserializers::Deserializer;
use crate::rest;
use crate::schedule::Schedule;
use crate::types::error::{LunchMenuError, ScheduleError, TokenError};
use crate::types::LunchMenu;
use crate::utils::{api, make_request};
use chrono::Duration;
use reqwest::Url;

pub use crate::types::{Org, Token, User, UserType};

impl User {
    /// Manually create a new user
    pub fn new(
        school_url: Url,
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

    /// Get a new token for the user
    ///
    /// This method uses the app key to get a new token from the schoolsoft api. The token is then
    /// used to authenticate to the api when making other requests.
    pub async fn get_token(&mut self) -> Result<Token, TokenError> {
        let url = rest!(self.school_url, token);

        let request = self
            .client
            .post(url)
            .header("appKey", &self.app_key)
            .header("deviceid", "TempleOs");

        let response = make_request(request)
            .await
            .map_err(TokenError::RequestError)?;

        let token = Token::deserialize(&response).map_err(TokenError::ParseError)?;

        self.token = Some(token.clone());

        Ok(token)
    }

    /// Get a token for the user
    ///
    /// Checks if the current token is close to expiring.
    /// If it is it will get a new token and replace [`Self::token`] with the new one.
    ///
    /// # Returns
    /// The [`Token`] or an error if a new token had to be fetched and failed.
    pub async fn smart_token(&mut self) -> Result<Token, TokenError> {
        match &self.token {
            Some(token) if token.is_safe() => Ok(token.clone()),
            _ => self.get_token().await,
        }
    }

    /// Get this weeks lunch menu
    ///
    /// # Returns
    /// A [`LunchMenu`] or [`LunchMenuError`] depending on if the request and parsing was
    /// successful
    pub async fn get_lunch(&mut self) -> Result<LunchMenu, LunchMenuError> {
        // Get token
        let token = self
            .smart_token()
            .await
            .map_err(LunchMenuError::TokenError)?;

        // Create Request
        let request = self
            .client
            .get(api(self, "lunchmenus"))
            .header("token", token.token);

        // Get menu
        let response = make_request(request)
            .await
            .map_err(LunchMenuError::RequestError)?;

        // Deserialize and return
        LunchMenu::deserialize(&response).map_err(LunchMenuError::ParseError)
    }

    /// Get the entire schedule (cus schoolsoft doesn't believe in the concept of filters)
    ///
    pub async fn get_schedule(&mut self) -> Result<Schedule, ScheduleError> {
        // Get token
        let token = self
            .smart_token()
            .await
            .map_err(ScheduleError::TokenError)?;

        // Create request
        let request = self
            .client
            .get(api(self, "lessons"))
            .header("token", token.token);

        let response = make_request(request)
            .await
            .map_err(ScheduleError::RequestError)?;

        Schedule::deserialize(&response).map_err(ScheduleError::ParseError)
    }
}

impl Token {
    /// Create a new token with a custom now function
    ///
    /// This is useful for testing as it allows you to set the current time to a fixed value
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::types::Token;
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

    /// Returns the duration until the token expires
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::types::Token;
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
    /// # use schoolsoft::types::Token;
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
    /// # use schoolsoft::types::Token;
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
    /// # use schoolsoft::types::Token;
    /// let token = Token::new_with_now(
    ///  "notrealtoken123_1337_1".to_string(),
    ///  chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(12, 0, 0).unwrap(),
    ///  || chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_hms_opt(11, 0, 0).unwrap(),
    /// );
    /// assert!(token.is_safe());
    /// ```
    /// Token with 30 sec left will return false
    /// ```
    /// # use schoolsoft::types::Token;
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
        use crate::deserializers::Deserializer;

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

            let user = User::deserialize(json_data).expect("Failed to deserialize JSON");

            assert_eq!(user.name, "Mock User");
            assert_eq!(
                user.pictute_url,
                "pictureFile.jsp?studentId=1337".to_string()
            );
            assert!(!user.is_of_age);
            assert_eq!(user.app_key, "123notreal".to_string());
            assert_eq!(user.token, None);
            assert_eq!(user.user_type, UserType::Student);
            assert_eq!(user.id, 1337);
            assert_eq!(user.orgs.len(), 1);

            let org = &user.orgs[0];
            assert_eq!(org.id, 1);
            assert_eq!(org.name, "Mock School");
            assert!(!org.blogger);
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
            assert!(!token.is_expired());
            assert!(token.is_valid());
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
            assert!(!token.is_expired());
            assert!(token.is_valid());
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
