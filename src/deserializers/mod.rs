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
///
/// In most cases this is just a wrapper around serde_json::from_str with some error handling.
/// But sometimes the deserialization process is a bit more involved than just taking the raw
/// values. In the future im going to make a custom serde deserializer instead but until then this
/// is a custom trait.
pub trait Deserializer {
    type Error;

    /// Used to convert the json string from an api request into its corresponding struct.
    fn deserialize(data: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Module with custom serde Deserializers for deserializing the weird dates that schoolsoft encode
/// time as.
///
/// They aren't that weird, its just that they are string representations with trailing milliseconds
/// and the default deserializer for chrono can't handle that.
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
