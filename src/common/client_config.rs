use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct ClientConfig {
    pub server_address: Arc<String>,
    pub access_token: Arc<String>,
    pub app_name: Arc<String>,
    pub ip: Arc<String>,
    pub port: u16,
    pub log_path: Arc<String>,
    pub log_retention_days: u32,
}

impl ClientConfig {
    pub fn get_http_addr(&self) -> String {
        format!("{}:{}", self.ip.as_str(), &self.port)
    }
}
