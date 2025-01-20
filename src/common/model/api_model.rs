use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobRunParam {
    pub job_id: u64,
    pub log_id: u64,
    pub executor_handler: Option<Arc<String>>,
    pub executor_params: Option<String>,
    pub executor_block_strategy: Option<String>,
    pub executor_timeout: Option<i32>,
    pub log_date_time: Option<u64>,
    pub glue_type: Option<String>,
    pub glue_source: Option<String>,
    #[serde(rename(serialize = "glueUpdatetime", deserialize = "glueUpdatetime"))]
    pub glue_update_time: Option<u64>,
    pub broadcast_index: Option<u64>,
    pub broadcast_total: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobIdleBeatParam {
    pub job_id: u64,
}
