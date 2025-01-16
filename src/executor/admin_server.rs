use crate::client::admin_client::AdminClient;
use crate::common::client_config::ClientConfig;
use actix::prelude::*;
use bean_factory::{bean, BeanFactory, FactoryData, Inject};
use std::sync::Arc;
use std::time::Duration;

#[bean(inject)]
pub struct ServerAccessActor {
    admin_client: Arc<AdminClient>,
}

impl ServerAccessActor {
    pub fn new(client_config: Arc<ClientConfig>) -> Self {
        let admin_client = Arc::new(AdminClient::new(client_config).unwrap());
        Self { admin_client }
    }

    fn registry_heartbeat(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::from_millis(29500), |act, ctx| {
            act.do_heartbeat(ctx);
            act.registry_heartbeat(ctx);
        });
    }

    fn do_heartbeat(&self, ctx: &mut Context<Self>) {
        let client = self.admin_client.clone();
        async move { client.registry().await }
            .into_actor(self)
            .map(|_res, act, ctx| {})
            .spawn(ctx);
    }
}

impl Actor for ServerAccessActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("ServerAccessActor started");
    }
}

impl Inject for ServerAccessActor {
    type Context = Context<Self>;

    fn inject(
        &mut self,
        _factory_data: FactoryData,
        _factory: BeanFactory,
        ctx: &mut Self::Context,
    ) {
        log::info!("ServerAccessActor inject");
        self.do_heartbeat(ctx);
        self.registry_heartbeat(ctx);
    }
}
