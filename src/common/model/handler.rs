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

/// 异步任务处理器；
/// 常规任务、io密集型任务推荐使用；
/// 只使用一个异步线程运行所有任务，不要在内容写同步堵塞线程逻辑；
#[async_trait]
pub trait AsyncJobHandler: Send + Sync {
    async fn process(&self, context: JobContext) -> anyhow::Result<JobContext>;
}

/// 同步任务处理器；
/// 每个任务起一个线程运行，CPU密集型任务推荐使用；
/// 任务数量多后线程数量不可控，后续考虑支持放到线程池运行；
pub trait SyncJobHandler: Send + Sync {
    fn process(&self, context: JobContext) -> anyhow::Result<JobContext>;
}

/// 任务处理器；
#[derive(Clone)]
pub enum JobHandler {
    /// 异步任务处理器；
    /// 常规任务、io密集型任务推荐使用；
    /// 只使用一个异步线程运行所有任务，不要在内容写同步堵塞线程逻辑；
    Async(Arc<dyn AsyncJobHandler>),
    /// 同步任务处理器；
    /// 每个任务起一个线程运行，CPU密集型任务推荐使用；
    /// 任务数量多后线程数量不可控，后续考虑支持放到线程池运行；
    Sync(Arc<dyn SyncJobHandler>),
}

impl JobHandler {
    pub fn is_async(&self) -> bool {
        match self {
            JobHandler::Async(_) => true,
            JobHandler::Sync(_) => false,
        }
    }

    pub fn get_async_handle(&self) -> Option<Arc<dyn AsyncJobHandler>> {
        match self {
            JobHandler::Async(h) => Some(h.clone()),
            JobHandler::Sync(_) => None,
        }
    }

    pub fn get_sync_handle(&self) -> Option<Arc<dyn SyncJobHandler>> {
        match self {
            JobHandler::Async(_) => None,
            JobHandler::Sync(h) => Some(h.clone()),
        }
    }
}

impl From<Arc<dyn AsyncJobHandler>> for JobHandler {
    fn from(value: Arc<dyn AsyncJobHandler>) -> Self {
        Self::Async(value)
    }
}

impl From<Arc<dyn SyncJobHandler>> for JobHandler {
    fn from(value: Arc<dyn SyncJobHandler>) -> Self {
        Self::Sync(value)
    }
}

#[derive(Clone)]
pub struct JobHandlerValue {
    pub handler: JobHandler,
    pub name: Arc<String>,
    pub is_running: bool,
    pub last_run_id: u64,
    pub block_jobs: Vec<JobContext>,
}

#[derive(Clone)]
pub struct JobHandlerRunParam {
    pub handler: JobHandler,
    pub name: Arc<String>,
}

impl JobHandlerValue {
    pub fn new(name: Arc<String>, handler: JobHandler) -> Self {
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
