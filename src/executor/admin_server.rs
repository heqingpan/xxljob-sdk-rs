use crate::client::admin_client::AdminClient;
use crate::common::client_config::ClientConfig;
use crate::common::model::admin_request::CallbackParam;
use crate::common::model::SUCCESS_CODE;
use crate::common::now_millis_i64;
use crate::executor::model::{ServerAccessActorReq, ServerAccessActorResult};
use actix::prelude::*;
use bean_factory::{bean, BeanFactory, FactoryData, Inject};
use std::sync::Arc;
use std::time::Duration;

#[bean(inject)]
pub struct ServerAccessActor {
    admin_client: Arc<AdminClient>,
    running: bool,
}

impl ServerAccessActor {
    pub fn new(client_config: Arc<ClientConfig>) -> Self {
        let admin_client = Arc::new(AdminClient::new(client_config).unwrap());
        Self {
            admin_client,
            running: false,
        }
    }

    fn registry_heartbeat(&self, ctx: &mut Context<Self>) {
        if !self.running {
            return;
        }
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

    fn stop(&mut self, ctx: &mut Context<Self>) {
        let client = self.admin_client.clone();
        async move { client.registry_remove().await }
            .into_actor(self)
            .map(|_res, act, ctx| {
                act.running = false;
                ctx.stop()
            })
            .wait(ctx);
    }

    fn callback(&self, params: Vec<CallbackParam>, ctx: &mut Context<Self>) {
        let client = self.admin_client.clone();
        async move { client.callback(params).await }
            .into_actor(self)
            .map(|_res, _act, _ctx| {})
            .spawn(ctx);
    }
}

impl Actor for ServerAccessActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("ServerAccessActor started");
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        if self.running {
            log::info!("ServerAccessActor stopping,return continue");
            self.stop(ctx);
            Running::Continue
        } else {
            log::info!("ServerAccessActor stopping,return Stop");
            Running::Stop
        }
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
        self.running = true;
        self.do_heartbeat(ctx);
        self.registry_heartbeat(ctx);
    }
}

impl Supervised for ServerAccessActor {}

impl Handler<ServerAccessActorReq> for ServerAccessActor {
    type Result = anyhow::Result<ServerAccessActorResult>;

    fn handle(&mut self, msg: ServerAccessActorReq, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ServerAccessActorReq::Stop => {
                self.stop(ctx);
            }
            ServerAccessActorReq::CallBack(params) => {
                self.callback(params, ctx);
            }
        };
        Ok(ServerAccessActorResult::None)
    }
}

pub(crate) fn do_callback(
    server_access_actor: &Addr<ServerAccessActor>,
    param: Vec<CallbackParam>,
) {
    server_access_actor.do_send(ServerAccessActorReq::CallBack(param));
}

pub fn callback_success(server_access_actor: &Addr<ServerAccessActor>, log_id: u64) {
    let callback_param = CallbackParam {
        log_id,
        log_date_tim: now_millis_i64(),
        handle_code: SUCCESS_CODE,
        handle_msg: None,
    };
    do_callback(server_access_actor, vec![callback_param]);
}

pub fn callback(
    server_access_actor: &Addr<ServerAccessActor>,
    log_id: u64,
    handle_code: i32,
    handle_msg: Option<String>,
) {
    let callback_param = CallbackParam {
        log_id,
        log_date_tim: now_millis_i64(),
        handle_code,
        handle_msg,
    };
    do_callback(server_access_actor, vec![callback_param]);
}
