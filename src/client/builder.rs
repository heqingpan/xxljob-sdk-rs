use crate::client::client::XxlClient;
use crate::common::actor_utils::create_actor_at_thread;
use crate::common::client_config::ClientConfig;
use crate::common::share_data::ShareData;
use crate::executor::core::ExecutorActor;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct ExecutorBuilder {
    server_address: String,
    access_token: Option<String>,
    app_name: Option<String>,
    ip: Option<String>,
    port: Option<u16>,
    log_path: Option<String>,
    log_retention_days: Option<u32>,
}

impl ExecutorBuilder {
    pub fn new(server_address: String) -> Self {
        Self {
            server_address,
            ..Default::default()
        }
    }

    pub fn set_access_token(mut self, access_token: String) -> Self {
        self.access_token = Some(access_token);
        self
    }

    pub fn set_app_name(mut self, app_name: String) -> Self {
        self.app_name = Some(app_name);
        self
    }

    pub fn set_ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    pub fn set_log_path(mut self, log_path: String) -> Self {
        self.log_path = Some(log_path);
        self
    }

    pub fn set_log_retention_days(mut self, log_retention_days: u32) -> Self {
        self.log_retention_days = Some(log_retention_days);
        self
    }

    pub fn build(self) -> XxlClient {
        let client_config = Arc::new(ClientConfig {
            server_address: Arc::new(self.server_address),
            access_token: Arc::new(self.access_token.unwrap_or_default()),
            app_name: Arc::new(self.app_name.unwrap_or_default()),
            ip: Arc::new(self.ip.unwrap_or_default()),
            port: self.port.unwrap_or_default(),
            log_path: Arc::new(self.log_path.unwrap_or_default()),
            log_retention_days: self.log_retention_days.unwrap_or_default(),
        });
        let executor_actor = create_actor_at_thread(ExecutorActor::new(client_config.clone()));
        let share_data = Arc::new(ShareData {
            client_config: client_config.clone(),
            executor_actor,
        });
        XxlClient::new(client_config, share_data)
    }
}
