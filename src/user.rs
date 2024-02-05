use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Clone, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum UserType {
    Student = 1,
    Parent = 2,
    Teacher = 3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Org {
    #[serde(rename = "orgId")]
    pub id: u32,
    pub name: String,
    pub blogger: bool,

    #[serde(rename = "schoolType")]
    pub school_type: u32,

    #[serde(rename = "leisureSchool")]
    pub leisure_school: u32,
    pub class: String,

    #[serde(rename = "tokenLogin")]
    pub token_login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub name: String,

    #[serde(rename = "pictureUrl")]
    pub pictute_url: String,

    #[serde(rename = "isOfAge")]
    pub is_of_age: bool,

    #[serde(rename = "appKey")]
    pub app_key: String,
    pub token: Option<String>,

    #[serde(rename = "type")]
    pub user_type: UserType,

    #[serde(rename = "userId")]
    pub id: u32,
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
