use crate::common::client_config::ClientConfig;
use crate::executor::admin_server::ServerAccessActor;
use crate::executor::core::ExecutorActor;
use actix::Addr;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ShareData {
    pub executor_actor: Addr<ExecutorActor>,
    pub server_access_actor: Addr<ServerAccessActor>,
    pub client_config: Arc<ClientConfig>,
}
