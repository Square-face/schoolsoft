pub mod lunch;
pub mod school;
pub mod user;

pub trait Deserializer {
    type Error;
    fn deserialize(data: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
