//! Deserializers for parsing api responses into structs.
//!
//! YOU SHOULD (probably) NOT BE HERE
//!
//! Under normal circumstances, you should not need to use this module directly.
//! But if you for example want to make a custom request to the SchoolSoft API,
//! then you have the option.

pub mod lunch;
pub mod school;
pub mod user;

/// Implemented to allow a struct to be deserialized from a json response
/// In most cases this is just a wrapper around serde_json::from_str with some error handling.
/// But some deserializers require more complex logic which is why this is a thing.
pub trait Deserializer {
    type Error;

    /// Used to convert the json string from an api request into its corresponding struct.
    fn deserialize(data: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

pub mod schoolsoft_date {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f").map_err(serde::de::Error::custom)
    }
}
