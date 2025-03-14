use crate::common::client_config::ClientConfig;
use crate::common::http_utils::{HttpUtils, ResponseWrap};
use crate::common::model::admin_request::{CallbackParam, RegistryParam};
use crate::common::model::XxlApiResult;
use crate::common::{constant, get_app_version};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct AdminClient {
    client_config: Arc<ClientConfig>,
    client: reqwest::Client,
    addrs: Vec<String>,
    headers: HashMap<String, String>,
}

impl AdminClient {
    pub fn new(client_config: Arc<ClientConfig>) -> anyhow::Result<Self> {
        let addrs_str = client_config.server_address.as_str();
        if addrs_str.is_empty() {
            return Err(anyhow::anyhow!("empty admin service address"));
        }
        let addrs = addrs_str
            .split(",")
            .filter(|&v| !v.is_empty())
            .map(|v| v.to_owned())
            .collect();
        let mut client_builder = reqwest::ClientBuilder::new();
        #[cfg(feature = "ssl_mode")]
        if client_config.ssl_danger_accept_invalid_certs {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }
        client_builder = client_builder.timeout(std::time::Duration::from_millis(3000));
        let client = client_builder.build()?;
        let mut headers = HashMap::new();
        if !client_config.access_token.is_empty() {
            headers.insert(
                "XXL-JOB-ACCESS-TOKEN".to_owned(),
                client_config.access_token.as_ref().clone(),
            );
            headers.insert("Content-Type".to_owned(), "application/json".to_owned());
            headers.insert(
                "User-Agent".to_owned(),
                format!("xxljob-sdk-rs/{}", get_app_version()),
            );
        }
        Ok(Self {
            client,
            addrs,
            client_config,
            headers,
        })
    }

    pub async fn registry(&self) -> anyhow::Result<()> {
        let address = format!(
            "http://{}:{}",
            self.client_config.ip, self.client_config.port
        );
        let param = RegistryParam {
            registry_group: constant::EXECUTOR.clone(),
            registry_key: self.client_config.app_name.clone(),
            registry_value: Arc::new(address),
        };
        let body = serde_json::to_vec(&param)?;
        match self.request(body, "registry").await {
            Ok(_) => {
                log::info!("admin_client|registry success");
                Ok(())
            }
            Err(e) => {
                log::error!("admin_client|registry error:{}", &e);
                Err(e)
            }
        }
    }

    pub async fn registry_remove(&self) -> anyhow::Result<()> {
        let address = format!(
            "http://{}:{}",
            self.client_config.ip, self.client_config.port
        );
        let param = RegistryParam {
            registry_group: constant::EXECUTOR.clone(),
            registry_key: self.client_config.app_name.clone(),
            registry_value: Arc::new(address),
        };
        let body = serde_json::to_vec(&param)?;
        match self.request(body, "registryRemove").await {
            Ok(_) => {
                log::info!("admin_client|registryRemove success");
                Ok(())
            }
            Err(e) => {
                log::error!("admin_client|registryRemove error:{}", &e);
                Err(e)
            }
        }
    }

    pub async fn callback(&self, params: &Vec<CallbackParam>) -> anyhow::Result<()> {
        let body = serde_json::to_vec(params)?;
        match self.request(body, "callback").await {
            Ok(_) => {
                log::info!("admin_client|callback success");
                Ok(())
            }
            Err(e) => {
                log::error!("admin_client|callback error:{}", &e);
                Err(e)
            }
        }
    }

    async fn request(&self, body: Vec<u8>, sub_url: &str) -> anyhow::Result<()> {
        let mut registry_success = false;
        for addr in &self.addrs {
            let url = format!("{}/api/{}", addr, &sub_url);
            match HttpUtils::request(
                &self.client,
                "POST",
                &url,
                body.clone(),
                Some(&self.headers),
                Some(3000),
            )
            .await
            {
                Ok(resp) => {
                    if let Ok(v) = Self::convert(&resp) {
                        if v.is_success() {
                            registry_success = true;
                            break;
                        }
                    }
                }
                Err(err) => {
                    log::error!("call response error:{},url:{}", err, &url);
                }
            }
        }
        if !registry_success {
            Err(anyhow::anyhow!("registry failed"))
        } else {
            Ok(())
        }
    }

    pub fn convert(resp: &ResponseWrap) -> anyhow::Result<XxlApiResult<String>> {
        let v = serde_json::from_slice(&resp.body)?;
        Ok(v)
    }
}
