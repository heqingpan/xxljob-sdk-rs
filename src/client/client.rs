use crate::common::model::handler::{AsyncJobHandler, JobHandler, JobHandlerValue, SyncJobHandler};
use crate::common::share_data::ShareData;
use crate::executor::model::ExecutorActorReq;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref LAST_XXL_CLIENT: Mutex<Option<Arc<XxlClient>>> =  Mutex::new(None);
}

pub fn set_last_xxl_client(client: Arc<XxlClient>) {
    if let Ok(mut r) = LAST_XXL_CLIENT.lock() {
        *r = Some(client);
    }
}

pub fn get_last_xxl_client() -> Option<Arc<XxlClient>> {
    if let Ok(r) = LAST_XXL_CLIENT.lock() {
        r.clone()
    } else {
        None
    }
}

/// xxl-job sdk客户端
#[derive(Clone)]
pub struct XxlClient {
    pub(crate) share_data: Arc<ShareData>,
}

impl XxlClient {
    pub(crate) fn new(share_data: Arc<ShareData>) -> Arc<XxlClient> {
        Arc::new(Self { share_data })
    }

    /*
    pub async fn stop(&self) -> anyhow::Result<()> {
        self.share_data
            .server_access_actor
            .send(ServerAccessActorReq::Stop)
            .await??;
        Ok(())
    }
    */

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
