use crate::errors::SchoolError;
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct RawSchoolListing {
    student_login_methods: String,
    parent_login_methods: String,
    teacher_login_methods: String,
    name: String,
    url: String,
}

impl SchoolListing {
    fn parse_methods(raw: &str) -> Result<Vec<u8>, SchoolError> {
        let mut methods = Vec::new();
        for method in raw.split(',') {
            if method.is_empty() {
                continue;
            }
            methods.push(method.parse().map_err(SchoolError::ParseError)?);
        }

        Ok(methods)
    }

    fn from_raw(raw_school: RawSchoolListing) -> Result<Self, SchoolError> {
        let login_methods = LoginMethods {
            student: Self::parse_methods(&raw_school.student_login_methods)?,
            teacher: Self::parse_methods(&raw_school.teacher_login_methods)?,
            parent: Self::parse_methods(&raw_school.parent_login_methods)?,
        };

        Ok(SchoolListing {
            login_methods,
            name: raw_school.name,
            url_name: raw_school
                .url
                .split('/')
                .nth_back(1)
                .ok_or(SchoolError::BadUrl)?
                .to_string(),
            url: raw_school.url,
        })
    }

    pub fn deserializer(json: &str) -> Result<Self, SchoolError> {
        let raw_school_listing: RawSchoolListing =
            serde_json::from_str(json).map_err(SchoolError::InvalidJson)?;

        Self::from_raw(raw_school_listing)
    }

    pub fn deserialize_many(json: &str) -> Result<Vec<Self>, SchoolError> {
        let raw_school_listings: Vec<RawSchoolListing> =
            serde_json::from_str(json).map_err(SchoolError::InvalidJson)?;

        let mut schools = Vec::new();

        for raw_school in raw_school_listings {
            let school = Self::from_raw(raw_school)?;

            schools.push(school);
        }

        Ok(schools)
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

        let school_listing =
            SchoolListing::deserializer(json_data).expect("Failed to deserialize JSON");

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

        let result = SchoolListing::deserializer(invalid_json);

        assert!(
            result.is_err(),
            "Expected an error when deserializing invalid JSON"
        );
    }
}
