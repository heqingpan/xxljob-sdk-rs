use crate::common::share_data::ShareData;
use crate::executor::model::ServerAccessActorReq;
use std::sync::Arc;

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
}
