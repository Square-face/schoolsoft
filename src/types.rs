use serde::de::{self, Deserializer, MapAccess, Visitor};
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

#[derive(Debug, Clone)]
pub struct LoginMethods {
    pub student: Vec<u8>,
    pub teacher: Vec<u8>,
    pub parent: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SchoolListing {
    pub login_methods: LoginMethods,
    pub name: String,
    pub url: String,
    pub url_name: String,
}

impl<'de> Deserialize<'de> for SchoolListing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SchoolListingVisitor;

        impl<'de> Visitor<'de> for SchoolListingVisitor {
            type Value = SchoolListing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map of strings")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                // Initialize the fields with default values.
                let mut student: Option<Vec<u8>> = None;
                let mut parent: Option<Vec<u8>> = None;
                let mut teacher: Option<Vec<u8>> = None;
                let mut name: Option<String> = None;
                let mut url: Option<String> = None;
                let mut url_name: Option<String> = None;

                // Iterate over the key-value pairs of the map.
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "studentLoginMethods" => {
                            student = Some(
                                map.next_value::<String>()?
                                    .split(',')
                                    .map(|s| s.parse::<u8>().unwrap())
                                    .collect(),
                            );
                        }
                        "parentLoginMethods" => {
                            parent = Some(
                                map.next_value::<String>()?
                                    .split(',')
                                    .map(|s| s.parse::<u8>().unwrap())
                                    .collect(),
                            );
                        }
                        "teacherLoginMethods" => {
                            teacher = Some(
                                map.next_value::<String>()?
                                    .split(',')
                                    .map(|s| s.parse::<u8>().unwrap())
                                    .collect(),
                            );
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "url" => {
                            let temp: String = map.next_value()?;
                            url = Some(temp.clone());

                            // Extract the URL name from the URL.
                            // Example: https://sms.schoolsoft.se/mock/ -> mock
                            url_name = temp.split('/').nth_back(1).map(|s| s.to_string());
                        }
                        field => {
                            let _ = map.next_value::<de::IgnoredAny>()?;
                            return Err(de::Error::unknown_field(
                                field,
                                &[
                                    "studentLoginMethods",
                                    "parentLoginMethods",
                                    "teacherLoginMethods",
                                    "name",
                                    "url",
                                ],
                            ));
                        }
                    }
                }

                // Ensure that all fields are initialized.
                let login_methods = LoginMethods {
                    student: student
                        .ok_or_else(|| de::Error::missing_field("studentLoginMethods"))?,
                    parent: parent.ok_or_else(|| de::Error::missing_field("parentLoginMethods"))?,
                    teacher: teacher
                        .ok_or_else(|| de::Error::missing_field("teacherLoginMethods"))?,
                };
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let url = url.ok_or_else(|| de::Error::missing_field("url"))?;
                let url_name = url_name.ok_or_else(|| de::Error::missing_field("url"))?;

                // Create and return the `SchoolListing`.
                Ok(SchoolListing {
                    login_methods,
                    name,
                    url,
                    url_name,
                })
            }
        }

        deserializer.deserialize_map(SchoolListingVisitor)
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

    mod list_schools {
        use super::*;

        #[test]
        fn deserialize_valid_json() {
            let json_data = r#"
            {
                "studentLoginMethods": "0,1,4",
                "parentLoginMethods": "4",
                "name": "Mock School",
                "teacherLoginMethods": "0",
                "url": "https://sms.schoolsoft.se/mock/"
            }
        "#;

            let school_listing: SchoolListing =
                serde_json::from_str(json_data).expect("Failed to deserialize JSON");

            assert_eq!(school_listing.name, "Mock School");
            assert_eq!(school_listing.url, "https://sms.schoolsoft.se/mock/");
            assert_eq!(school_listing.url_name, "mock");
            assert_eq!(school_listing.login_methods.student, vec![0, 1, 4]);
            assert_eq!(school_listing.login_methods.teacher, vec![0]);
            assert_eq!(school_listing.login_methods.parent, vec![4]);
        }

        #[test]
        fn deserialize_invalid_json() {
            // Test with invalid JSON data
            let invalid_json = r#"
            {
                "studentLoginMethods": "0,4",
                "parentLoginMethods": "0,4",
                "name": "Carl Wahren Gymnasium",
                "teacherLoginMethods": "0,4",
                "url": "https://sms.schoolsoft.se/carlwahren/",
                "extraField": "extraValue"
            }
        "#;

            let result: Result<SchoolListing, _> = serde_json::from_str(invalid_json);

            assert!(
                result.is_err(),
                "Expected an error when deserializing invalid JSON"
            );
        }
    }
}
