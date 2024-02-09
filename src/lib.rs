use errors::{RequestError, SchoolParseError};
use school::SchoolListing;

pub mod errors;
pub mod school;
pub mod user;
mod utils;

/// Api client for the api used by schoolsofts app
#[derive(Debug)]
pub struct Client {
    /// Http client used for requests.
    client: reqwest::Client,

    /// Url to put before all requests.
    /// Default: <https://sms.schoolsoft.se>
    ///
    /// Can be modified for testing purposes.
    base_url: String,

    /// Device id to use when requesting a token.
    /// Default: ""
    ///
    /// Requesting a token requires a device id. The id can be "" without causing any known issues.
    ///
    /// Its presumablly used by schoolsoft for logging and analytics.
    device_id: String,

    /// Some(user) if the client is logged in.
    /// None if the client is not logged in.
    ///
    /// Methods that require a logged in user will return an error if the user is not logged in.
    pub user: Option<user::User>,
}

/// Client builder.
///
/// Useful for configuring the client.
///
/// The default values are:
/// - base_url: <https://sms.schoolsoft.se>
/// - device_id: ""
///
/// # Examples
///
/// construct a client with a custom base url and device device_id
///
/// ```
/// # use schoolsoft::ClientBuilder;
/// let client = ClientBuilder::new()
///   .base_url("https://example.com".to_string())
///   .device_id("1234567890".to_string())
///   .build();
///
/// assert_eq!(client.base_url(), "https://example.com");
/// assert_eq!(client.device_id(), "1234567890");
/// ```
///
/// construct a client with a custom base url
///
/// ```
/// # use schoolsoft::ClientBuilder;
/// let client = ClientBuilder::new()
///    .base_url("https://example.com".to_string())
///    .build();
/// assert_eq!(client.base_url(), "https://example.com");
/// assert_eq!(client.device_id(), "");
/// ```
///
/// construct a client with a custom device_id
///
/// ```
/// # use schoolsoft::ClientBuilder;
/// let client = ClientBuilder::new()
///    .device_id("1234567890".to_string())
///    .build();
///
/// assert_eq!(client.base_url(), "https://sms.schoolsoft.se");
/// assert_eq!(client.device_id(), "1234567890");
/// ```
///
///
/// construct a client with the default values
///
/// ```
/// # use schoolsoft::ClientBuilder;
/// let client = ClientBuilder::new()
///     .build();
///
/// assert_eq!(client.base_url(), "https://sms.schoolsoft.se");
/// assert_eq!(client.device_id(), "");
/// ```
///
#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    device_id: Option<String>,
}

impl Client {
    /// Attempt to login with the given credentials.
    ///
    /// `school` is the [school::SchoolListing::url_name] of the school.
    /// username and password are the same as when logging into the website or mobile app
    ///
    /// # Arguments
    /// - username: The username to login with.
    /// - password: The password to login with.
    /// - school: The school to login to.
    ///
    /// # Examples
    ///
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// # use tokio::test;
    ///
    /// # #[test]
    /// # async fn login() {
    /// let mut client = ClientBuilder::new()
    ///    .build();
    ///
    /// client.login("username", "password", "school").await;
    /// # }
    /// ```
    ///
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
        school: &str,
    ) -> Result<(), RequestError<serde_json::Error>> {
        // Construct url
        let url = format!("{}/{}/rest/app/login", self.base_url, school);

        // Construct body
        let mut params = std::collections::HashMap::new();
        params.insert("identification", username);
        params.insert("verification", password);
        params.insert("logintype", "4");
        params.insert("usertype", "1");

        let map_err = self
            // build request
            .client
            .request(reqwest::Method::POST, url)
            .form(&params)
            // send request
            .send()
            .await
            // Check if the request was successful
            .map_err(RequestError::RequestError)
            // Check response code
            .and_then(|response| utils::check_codes(response.status()).map_or(Ok(response), Err))?
            // Read response body
            .text()
            .await
            // Check if the response body was read successfully
            .map_err(RequestError::ReadError)?;

        // Parse response
        let user = user::User::deserialize(&map_err).map_err(RequestError::ParseError)?;

        self.user = Some(user);
        Ok(())
    }

    /// Get list of schools.
    ///
    /// Returns every school currently available in the system.
    ///
    /// # Examples
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// # use tokio::test;
    /// #
    /// # #[test]
    /// # async fn schools() {
    /// let client = ClientBuilder::new()
    ///   .build();
    ///
    /// let schools = client.schools().await;
    /// # }
    pub async fn schools(
        &self,
    ) -> Result<Vec<school::SchoolListing>, RequestError<SchoolParseError>> {
        let url = format!("{}/rest/app/schoollist/prod", self.base_url);

        SchoolListing::deserialize_many(
            &self
                .client
                .get(&url)
                .send()
                .await
                .map_err(RequestError::RequestError)
                .and_then(|response| {
                    let code = response.status();
                    response
                        .status()
                        .is_success()
                        .then_some(response)
                        .ok_or(RequestError::UncheckedCode(code))
                })?
                .text()
                .await
                .map_err(RequestError::ReadError)?,
        )
    }

    /// Get the base url.
    ///
    /// # Examples
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// let client = ClientBuilder::new()
    ///   .base_url("https://example.com".to_string())
    ///   .build();
    ///
    /// assert_eq!(client.base_url(), "https://example.com");
    /// ```
    pub fn base_url(&self) -> &str {
        self.base_url.as_ref()
    }

    /// Get the device id.
    ///
    /// # Examples
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// let client = ClientBuilder::new()
    ///   .device_id("1234567890".to_string())
    ///   .build();
    ///
    /// assert_eq!(client.device_id(), "1234567890");
    pub fn device_id(&self) -> &str {
        self.device_id.as_ref()
    }
}

impl ClientBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base url.
    ///
    /// Default: <https://sms.schoolsoft.se>
    ///
    /// # Examples
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// let client = ClientBuilder::new()
    ///    .base_url("https://example.com".to_string())
    ///    .build();
    ///
    /// assert_eq!(client.base_url(), "https://example.com");
    /// ```
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Set the device id.
    ///
    /// Default: ""
    ///
    /// # Examples
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// let client = ClientBuilder::new()
    ///   .device_id("1234567890".to_string())
    ///   .build();
    ///
    /// assert_eq!(client.device_id(), "1234567890");
    /// ```
    pub fn device_id(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn build(self) -> Client {
        Client {
            client: reqwest::Client::new(),
            base_url: self
                .base_url
                .unwrap_or("https://sms.schoolsoft.se".to_string()),
            device_id: self.device_id.unwrap_or("".to_string()),
            user: None,
        }
    }
}

#[cfg(test)]
mod client_builder_tests {
    use super::*;

    #[test]
    fn with_base_url() {
        let client = ClientBuilder::new()
            .base_url("https://sms.schoolsoft.se".to_string())
            .build();

        assert_eq!(client.base_url(), "https://sms.schoolsoft.se");
    }
}
