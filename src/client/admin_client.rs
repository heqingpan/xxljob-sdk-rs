use crate::common::client_config::ClientConfig;
use crate::common::constant;
use crate::common::http_utils::{HttpUtils, ResponseWrap};
use crate::common::model::admin_request::RegistryParam;
use crate::common::model::XxlApiResult;
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
        let client = reqwest::Client::new();
        let mut headers = HashMap::new();
        if !client_config.access_token.is_empty() {
            headers.insert(
                "XXL-JOB-ACCESS-TOKEN".to_owned(),
                client_config.access_token.as_ref().clone(),
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
        let param = RegistryParam {
            registry_group: self.client_config.app_name.clone(),
            registry_key: self.client_config.app_name.clone(),
            registry_value: constant::EXECUTOR.clone(),
        };
        let body = serde_json::to_vec(&param)?;
        let mut registry_success = false;
        for addr in &self.addrs {
            let url = format!("http://{}/api/registry", addr);
            if let Ok(resp) = HttpUtils::request(
                &self.client,
                "post",
                &url,
                body.clone(),
                Some(&self.headers),
                None,
            )
            .await
            {
                if let Ok(v) = Self::convert(&resp) {
                    if v.is_success() {
                        registry_success = true;
                        break;
                    }
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
