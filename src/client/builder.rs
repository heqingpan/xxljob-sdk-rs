use crate::client::client::{set_last_xxl_client, XxlClient};
use crate::common::actor_utils::create_actor_at_thread;
use crate::common::client_config::ClientConfig;
use crate::common::ip_utils::{get_available_port, get_local_ip};
use crate::common::share_data::ShareData;
use crate::executor::admin_server::ServerAccessActor;
use crate::executor::core::ExecutorActor;
use crate::server::web_server::ServerRunner;
use actix::Actor;
use actix_rt::System;
use bean_factory::{BeanDefinition, BeanFactory};
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct XxlClientBuilder {
    server_address: String,
    access_token: Option<String>,
    app_name: Option<String>,
    ip: Option<String>,
    port: Option<u16>,
    log_path: Option<String>,
    log_retention_days: Option<u32>,
    ssl_danger_accept_invalid_certs: Option<bool>,
}

impl XxlClientBuilder {
    pub fn new(server_address: String) -> Self {
        Self {
            server_address,
            ..Default::default()
        }
    }

    /// 设置访问token要与服务端的token一致
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

    pub fn set_ssl_danger_accept_invalid_certs(
        mut self,
        ssl_danger_accept_invalid_certs: bool,
    ) -> Self {
        self.ssl_danger_accept_invalid_certs = Some(ssl_danger_accept_invalid_certs);
        self
    }

    pub fn build(self) -> anyhow::Result<Arc<XxlClient>> {
        let start_port = 9900;
        let port = Self::get_port(start_port, self.port);
        if port == 0 {
            return Err(anyhow::anyhow!("no available port"));
        }
        let client_config = Arc::new(ClientConfig {
            server_address: Arc::new(self.server_address),
            access_token: Arc::new(self.access_token.unwrap_or_default()),
            app_name: Arc::new(self.app_name.unwrap_or("unknown".to_string())),
            ip: Arc::new(self.ip.unwrap_or(get_local_ip())),
            port,
            log_path: Arc::new(self.log_path.unwrap_or_default()),
            log_retention_days: self.log_retention_days.unwrap_or_default(),
            ssl_danger_accept_invalid_certs: self.ssl_danger_accept_invalid_certs.unwrap_or(true),
        });
        if client_config.access_token.is_empty() {
            log::warn!("api access_token is empty!");
        }
        let client = build_client(client_config)?;
        set_last_xxl_client(client.clone());
        Ok(client)
    }

    fn get_port(start_port: u16, option_port: Option<u16>) -> u16 {
        let source_port = option_port.unwrap_or_default();
        let port = if source_port == 0 {
            get_available_port(start_port)
        } else {
            get_available_port(source_port)
        };
        if source_port != port {
            log::info!("auto use port: {}", port);
        } else {
            log::info!("use set port: {}", source_port);
        }
        port
    }
}

fn build_client(client_config: Arc<ClientConfig>) -> anyhow::Result<Arc<XxlClient>> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let rt = System::new();
        let r = rt.block_on(async_init(client_config));
        tx.send(r).unwrap();
        rt.run().unwrap();
    });
    rx.recv()?
}

fn init_factory(client_config: Arc<ClientConfig>) -> anyhow::Result<BeanFactory> {
    let factory = BeanFactory::new();
    factory.register(BeanDefinition::actor_with_inject_from_obj(
        //ExecutorActor::new(client_config.clone()).start(),
        create_actor_at_thread(ExecutorActor::new(client_config.clone())),
    ));
    factory.register(BeanDefinition::actor_with_inject_from_obj(
        ServerRunner {}.start(),
    ));
    factory.register(BeanDefinition::actor_with_inject_from_obj(
        ServerAccessActor::new(client_config.clone()).start(),
    ));
    factory.register(BeanDefinition::from_obj(client_config.clone()));
    Ok(factory)
}

async fn async_init(client_config: Arc<ClientConfig>) -> anyhow::Result<Arc<XxlClient>> {
    let factory = init_factory(client_config.clone())?;
    let factory_data = factory.init().await;
    let share_data = Arc::new(ShareData {
        executor_actor: factory_data.get_actor().unwrap(),
        server_access_actor: factory_data.get_actor().unwrap(),
        client_config,
    });
    let client = XxlClient::new(share_data.clone());
    /*
    tokio::spawn(async move {
        run_embed_web(share_data).await.ok();
    });
     */
    Ok(client)
}
