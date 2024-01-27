
pub mod types;

/// Api client for the api used by schoolsofts app
pub struct Client {
    client: reqwest::Client,
    /// Url to put before all requests.
    /// Default: <https://sms.schoolsoft.se/>
    ///
    /// Can be modified for testing purposes.
    base_url: String,

    /// Device id to use when requesting a token.
    /// Default: ""
    ///
    /// Requesting a token requires a device id. The id can be "" without causing any known issues.
    ///
    /// Its presumablly used for logging and analytics.
    device_id: String,

    /// Some(user) if the client is logged in.
    /// None if the client is not logged in.
    pub user: Option<types::User>,
}

/// Client builder.
///
/// Usefull for configuring the client.
///
/// The default values are:
/// - base_url: <https://sms.schoolsoft.se/>
/// - device_id: ""
///
/// # Examples
///
/// construct a client with a custom base url and device device_id
///
/// ```
/// # use schoolsoft::ClientBuilder;
/// let client = ClientBuilder::new()
///   .base_url("https://example.com/".to_string())
///   .device_id("1234567890".to_string())
///   .build();
///
/// assert_eq!(client.base_url(), "https://example.com/");
/// assert_eq!(client.device_id(), "1234567890");
/// ```
///
/// construct a client with a custom base url
///
/// ```
/// # use schoolsoft::ClientBuilder;
/// let client = ClientBuilder::new()
///    .base_url("https://example.com/".to_string())
///    .build();
/// assert_eq!(client.base_url(), "https://example.com/");
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
/// assert_eq!(client.base_url(), "https://sms.schoolsoft.se/");
/// assert_eq!(client.device_id(), "1234567890");
/// ```
///
/// construct a client with the default values
///
/// ```
/// # use schoolsoft::ClientBuilder;
/// let client = ClientBuilder::new()
///     .build();
///
/// assert_eq!(client.base_url(), "https://sms.schoolsoft.se/");
/// assert_eq!(client.device_id(), "");
/// ```
///
pub struct ClientBuilder {
    base_url: Option<String>,
    device_id: Option<String>,
}

impl Client {

    /// Get the base url.
    ///
    /// # Examples
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// let client = ClientBuilder::new()
    ///   .base_url("https://example.com/".to_string())
    ///   .build();
    ///
    /// assert_eq!(client.base_url(), "https://example.com/");
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
    /// Default: <https://sms.schoolsoft.se/>
    ///
    /// # Examples
    /// ```
    /// # use schoolsoft::ClientBuilder;
    /// let client = ClientBuilder::new()
    ///    .base_url("https://example.com/".to_string())
    ///    .build();
    ///
    /// assert_eq!(client.base_url(), "https://example.com/");
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
            base_url: self.base_url.unwrap_or("https://sms.schoolsoft.se/".to_string()),
            device_id: self.device_id.unwrap_or("".to_string()),
            user: None,
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: None,
            device_id: None,
        }
    }
}

#[cfg(test)]
mod client_builder_tests {
    use super::*;

    #[test]
    fn with_base_url() {
        let client = ClientBuilder::new()
            .base_url("https://sms.schoolsoft.se/".to_string())
            .build();

        assert_eq!(client.base_url(), "https://sms.schoolsoft.se/");
    }
}
