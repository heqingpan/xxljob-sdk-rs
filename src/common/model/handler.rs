use crate::common::model::api_model::JobRunParam;
use crate::common::model::enum_type::{ExecutorBlockStrategy, GlueType};
use crate::common::model::{FAIL_CODE, SUCCESS_CODE};
use crate::common::share_data::ShareData;
use crate::executor::admin_server;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct JobContext {
    pub job_id: u64,
    pub job_param: Option<String>,
    pub job_log_file_name: Option<String>,
    pub log_id: u64,
    pub shard_index: u64,
    pub shard_total: u64,
    pub handle_code: i32,
    pub handle_msg: Option<String>,
    pub block_strategy: ExecutorBlockStrategy,
    pub glue_type: GlueType,
    pub(crate) share_data: Arc<ShareData>,
}

impl JobContext {
    pub fn new(run_param: JobRunParam, share_data: Arc<ShareData>) -> Self {
        Self {
            job_id: run_param.job_id,
            job_param: run_param.executor_params,
            job_log_file_name: None,
            log_id: run_param.log_id,
            shard_index: run_param.broadcast_index.unwrap_or(0),
            shard_total: run_param.broadcast_total.unwrap_or(1),
            glue_type: GlueType::from_str(&run_param.glue_type.unwrap_or_default())
                .unwrap_or(GlueType::Bean),
            block_strategy: ExecutorBlockStrategy::from_str(
                &run_param.executor_block_strategy.unwrap_or_default(),
            ),
            handle_code: SUCCESS_CODE,
            handle_msg: None,
            share_data,
        }
    }

    pub fn callback_success(&self) {
        admin_server::callback_success(&self.share_data.server_access_actor, self.log_id);
    }

    pub fn callback_failed(&self) {
        let handle_code = if self.handle_code != SUCCESS_CODE {
            self.handle_code
        } else {
            FAIL_CODE
        };
        admin_server::callback(
            &self.share_data.server_access_actor,
            self.log_id,
            handle_code,
            self.handle_msg.clone(),
        );
    }

    pub fn callback_failed_with_info(&self, error_msg: String, handle_code: i32) {
        admin_server::callback(
            &self.share_data.server_access_actor,
            self.log_id,
            handle_code,
            Some(error_msg),
        );
    }
}

#[async_trait]
pub trait JobHandler: Send + Sync {
    async fn process(&self, context: JobContext) -> anyhow::Result<JobContext>;
}

#[derive(Clone)]
pub struct JobHandlerValue {
    pub handler: Arc<dyn JobHandler>,
    pub name: Arc<String>,
    pub is_running: bool,
    pub last_run_id: u64,
    pub block_jobs: Vec<JobContext>,
}

#[derive(Clone)]
pub struct JobHandlerRunParam {
    pub handler: Arc<dyn JobHandler>,
    pub name: Arc<String>,
}

impl JobHandlerValue {
    pub fn new(name: Arc<String>, handler: Arc<dyn JobHandler>) -> Self {
        Self {
            handler,
            name,
            is_running: false,
            last_run_id: 0,
            block_jobs: Vec::with_capacity(2),
        }
    }
    pub fn push_block_job(&mut self, job: JobContext) -> Option<JobContext> {
        if !self.is_running {
            return None;
        }
        if self.block_jobs.len() >= 10 {
            let remove = self.block_jobs.remove(0);
            self.block_jobs.push(job);
            Some(remove)
        } else {
            self.block_jobs.push(job);
            None
        }
    }

    pub fn pop_block_job(&mut self) -> Option<JobContext> {
        if self.block_jobs.is_empty() {
            None
        } else {
            Some(self.block_jobs.remove(0))
        }
    }

    pub fn build_run_param(&self) -> JobHandlerRunParam {
        JobHandlerRunParam {
            handler: self.handler.clone(),
            name: self.name.clone(),
        }
    }
}
