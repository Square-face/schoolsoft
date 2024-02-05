use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;

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
                                serde_json::from_str::<Vec<u8>>(&format!(
                                    "[{}]",
                                    &map.next_value::<String>()?
                                ))
                                .map_err(de::Error::custom)?,
                            );
                        }

                        "parentLoginMethods" => {
                            parent = Some(
                                serde_json::from_str::<Vec<u8>>(&format!(
                                    "[{}]",
                                    &map.next_value::<String>()?
                                ))
                                .map_err(de::Error::custom)?,
                            );
                        }
                        "teacherLoginMethods" => {
                            teacher = Some(
                                serde_json::from_str::<Vec<u8>>(&format!(
                                    "[{}]",
                                    &map.next_value::<String>()?
                                ))
                                .map_err(de::Error::custom)?,
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
