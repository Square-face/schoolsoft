use serde::Deserialize;
use serde_repr::Deserialize_repr;

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
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Token {
    /// The token itself
    pub token: String,

    /// When the token expires
    pub expires: chrono::DateTime<chrono::Utc>,
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
#[derive(Debug, Clone, Deserialize)]
pub struct Org {
    /// Unique identifier for the organization
    #[serde(rename = "orgId")]
    pub id: u32,

    /// Human readable name of the organization
    pub name: String,

    /// Unknown
    pub blogger: bool,

    /// Unknown
    #[serde(rename = "schoolType")]
    pub school_type: u32,

    /// Unknown, also, why is it a number?
    #[serde(rename = "leisureSchool")]
    pub leisure_school: u32,

    /// If we assume that this is a school, then this is the class that the user is attending
    /// But what about teachers and parents? What does this field mean for them?
    pub class: String,

    /// Url to login to the organization using a web browser
    /// Once again, this field makes no since as you get it by logging in, so why would you need to
    /// log in again?
    #[serde(rename = "tokenLogin")]
    pub token_login: String,
}

/// A schoolsoft user
///
/// This struct represents a user of the schoolsoft system. It is deserialized from the JSON
/// returned by the api when logging in.
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    /// Users full name
    pub name: String,

    /// Url to the users profile picture
    #[serde(rename = "pictureUrl")]
    pub pictute_url: String,

    /// If the user is over 18 (schoolsoft is swedish)
    #[serde(rename = "isOfAge")]
    pub is_of_age: bool,

    /// The app key retrieved when logging in
    #[serde(rename = "appKey")]
    pub app_key: String,

    /// Token used for interacting with api routes that require authentication
    ///
    /// This field is not populated by logging in. Instead it requires a separate request to
    /// /<school>/rest/app/token with the app key.
    pub token: Option<Token>,

    /// What type of user this is
    #[serde(rename = "type")]
    pub user_type: UserType,

    /// Unique identifier for the user
    #[serde(rename = "userId")]
    pub id: u32,

    /// List of organizations that the user is a part of
    pub orgs: Vec<Org>,
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

            let user: User = serde_json::from_str(json_data).expect("Failed to deserialize JSON");

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

}
