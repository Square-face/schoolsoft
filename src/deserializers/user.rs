use super::Deserializer;
use crate::{
    types::{Org, User, UserType},
    user::Token,
};
use reqwest::Url;
use serde::de::Error;
use serde::Deserialize;

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

impl Deserializer for User {
    type Error = serde_json::Error;

    /// Deserialize a user from a JSON String
    ///
    /// # Arguments
    /// * `json` - A JSON string containing the user data
    /// * `school_url` - The url of the school the user is logging in to
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::types::User;
    /// # use schoolsoft::deserializers::Deserializer;
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
    /// ).expect("Failed to deserialize JSON");
    /// ```
    fn deserialize(json: &str) -> Result<User, serde_json::Error> {
        let raw: RawUser = serde_json::from_str(json)?;

        // This is technically unnecessary but allows for future functionality to be added to Org
        let orgs: Vec<Org> = raw.orgs.into_iter().map(Org::from).collect();

        // There must be a better way to only get the first part of the path without any query args or fragments
        let mut school_url = Url::parse(&orgs[0].token_login).unwrap();
        let segments = school_url.clone();
        school_url.set_path(segments.path_segments().unwrap().next().unwrap());
        school_url.set_query(None);
        school_url.set_fragment(None);

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
}

impl Token {
    /// Deserialize a token from a JSON string
    ///
    /// # Example
    /// ```
    /// # use schoolsoft::types::Token;
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

impl From<RawOrg> for Org {
    fn from(value: RawOrg) -> Self {
        Org {
            id: value.org_id,
            name: value.name,
            blogger: value.blogger,
            school_type: value.school_type,
            leisure_school: value.leisure_school,
            class: value.class,
            token_login: value.token_login,
        }
    }
}
