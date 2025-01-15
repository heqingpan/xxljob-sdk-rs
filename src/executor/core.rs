use crate::common::client_config::ClientConfig;
use actix::prelude::*;
use std::sync::Arc;

#[derive(Clone, Debug, Default)]
pub struct ExecutorActor {
    client_config: Arc<ClientConfig>,
}

impl ExecutorActor {
    pub fn new(client_config: Arc<ClientConfig>) -> Self {
        Self { client_config }
    }
}

impl Actor for ExecutorActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Executor actor started");
    }
}
