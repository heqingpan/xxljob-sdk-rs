use crate::common::model::handler::{AsyncJobHandler, JobHandler, JobHandlerValue, SyncJobHandler};
use crate::common::share_data::ShareData;
use crate::executor::model::{ExecutorActorReq, ServerAccessActorReq};
use std::sync::Arc;

/// xxl-job sdk客户端
pub struct XxlClient {
    pub(crate) share_data: Arc<ShareData>,
    is_running: bool,
}

impl XxlClient {
    pub(crate) fn new(share_data: Arc<ShareData>) -> XxlClient {
        Self {
            share_data,
            is_running: false,
        }
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        self.share_data
            .server_access_actor
            .send(ServerAccessActorReq::Stop)
            .await??;
        Ok(())
    }

    /// 注册任务
    pub fn register(&self, job_name: Arc<String>, job_handler: JobHandler) -> anyhow::Result<()> {
        self.share_data
            .executor_actor
            .do_send(ExecutorActorReq::Register(JobHandlerValue::new(
                job_name,
                job_handler,
            )));
        Ok(())
    }

    /// 注册任务
    pub fn register_async(
        &self,
        job_name: Arc<String>,
        job_handler: Arc<dyn AsyncJobHandler>,
    ) -> anyhow::Result<()> {
        self.share_data
            .executor_actor
            .do_send(ExecutorActorReq::Register(JobHandlerValue::new(
                job_name,
                job_handler.into(),
            )));
        Ok(())
    }

    pub fn register_sync(
        &self,
        job_name: Arc<String>,
        job_handler: Arc<dyn SyncJobHandler>,
    ) -> anyhow::Result<()> {
        self.share_data
            .executor_actor
            .do_send(ExecutorActorReq::Register(JobHandlerValue::new(
                job_name,
                job_handler.into(),
            )));
        Ok(())
    }
}
