use serde::ser::{Serialize, SerializeStruct};

use crate::types::{LoginMethods, Lunch, LunchMenu, SchoolListing};

impl Serialize for LoginMethods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("LoginMethods", 3)?;
        state.serialize_field("student", &self.student)?;
        state.serialize_field("teacher", &self.teacher)?;
        state.serialize_field("parent", &self.parent)?;
        state.end()
    }
}

impl Serialize for SchoolListing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("SchoolListing", 4)?;
        state.serialize_field("login_methods", &self.login_methods)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("url_name", &self.url_name)?;
        state.end()
    }
}

impl Serialize for Lunch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Lunch", 2)?;
        state.serialize_field("date", &self.date)?;
        state.serialize_field("food", &self.food)?;
        state.end()
    }
}

impl Serialize for LunchMenu {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("LunchMenu", 8)?;
        state.serialize_field("week", &self.week)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("category", &self.category)?;
        state.serialize_field("monday", &self.monday)?;
        state.serialize_field("tuesday", &self.tuesday)?;
        state.serialize_field("wednesday", &self.wednesday)?;
        state.serialize_field("thursday", &self.thursday)?;
        state.serialize_field("friday", &self.friday)?;
        state.end()
    }
}

#[cfg(test)]
mod login_methods {
    use serde_json::json;

    use crate::types::LoginMethods;

    #[test]
    fn serialize() {
        let login_methods = LoginMethods {
            student: vec![1, 2, 3],
            teacher: vec![4, 5, 6],
            parent: vec![7, 8, 9],
        };

        let expected = json!({
            "student": [1, 2, 3],
            "teacher": [4, 5, 6],
            "parent": [7, 8, 9],
        });

        let serialized = serde_json::to_value(login_methods).unwrap();
        assert_eq!(serialized, expected);
    }
}

#[cfg(test)]
mod school_listing {
    use serde_json::json;
    use pretty_assertions::assert_eq;

    use crate::types::{LoginMethods, SchoolListing};

    #[test]
    fn serialize() {
        let school_listing = SchoolListing {
            login_methods: LoginMethods {
                student: vec![],
                teacher: vec![],
                parent: vec![],
            },
            name: "Example School".to_string(),
            url: "https://example.com/example_school".to_string(),
            url_name: "example_school".to_string(),
        };

        let expected = json!({
            "login_methods": {
                "student": [],
                "teacher": [],
                "parent": [],
            },
            "name": "Example School",
            "url": "https://example.com/example_school",
            "url_name": "example_school",
        });

        let serialized = serde_json::to_value(school_listing).unwrap();
        assert_eq!(serialized, expected);
    }
}

#[cfg(test)]
mod lunch {

    use serde_json::json;
    use pretty_assertions::assert_eq;

    use crate::types::{Lunch, LunchMenu};

    #[test]
    #[allow(deprecated)]
    fn lunch() {
        let lunch = Lunch {
            date: chrono::NaiveDate::from_ymd(2021, 1, 1),
            food: "Example food".to_string(),
        };

        let expected = json!({
            "date": "2021-01-01",
            "food": "Example food",
        });

        let serialized = serde_json::to_value(lunch).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    #[allow(deprecated)]
    fn lunch_menu() {
        let lunch_menu = LunchMenu {
            week: 1,
            created_at: chrono::NaiveDate::from_ymd(2021, 1, 1).and_hms(0, 0, 0),
            category: "Example category".to_string(),
            monday: Lunch {
                date: chrono::NaiveDate::from_ymd(2021, 1, 1),
                food: "Foo".to_string(),
            },
            tuesday: Lunch {
                date: chrono::NaiveDate::from_ymd(2021, 1, 2),
                food: "Goo".to_string(),
            },
            wednesday: Lunch {
                date: chrono::NaiveDate::from_ymd(2021, 1, 3),
                food: "Roo".to_string(),
            },
            thursday: Lunch {
                date: chrono::NaiveDate::from_ymd(2021, 1, 4),
                food: "Zoo".to_string(),
            },
            friday: Lunch {
                date: chrono::NaiveDate::from_ymd(2021, 1, 5),
                food: "Hoo".to_string(),
            },
        };

        let expected = json!({
            "week": 1,
            "created_at": "2021-01-01T00:00:00",
            "category": "Example category",
            "monday": {
                "date": "2021-01-01",
                "food": "Foo",
            },
            "tuesday": {
                "date": "2021-01-02",
                "food": "Goo",
            },
            "wednesday": {
                "date": "2021-01-03",
                "food": "Roo",
            },
            "thursday": {
                "date": "2021-01-04",
                "food": "Zoo",
            },
            "friday": {
                "date": "2021-01-05",
                "food": "Hoo",
            },
        });

        let serialized = serde_json::to_value(lunch_menu).unwrap();
        assert_eq!(serialized, expected);
    }
}
