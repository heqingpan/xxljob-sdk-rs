use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegistryParam {
    pub registry_group: Arc<String>,
    pub registry_key: Arc<String>,
    pub registry_value: Arc<String>,
}
