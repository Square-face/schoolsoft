pub mod types;

pub struct Client {
    client: reqwest::Client,
    pub base_url: String,
    pub device_id: String,
    pub user: Option<types::User>,
    pub os: String,
    pub token: Option<String>,
    pub app_id: Option<String>,
}

