use crate::common::client_config::ClientConfig;
use crate::executor::core::ExecutorActor;
use actix::Addr;
use std::sync::Arc;

pub struct ShareData {
    pub executor_actor: Addr<ExecutorActor>,
    pub client_config: Arc<ClientConfig>,
}
