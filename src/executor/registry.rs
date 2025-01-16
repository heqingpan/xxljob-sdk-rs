use actix::prelude::*;

pub struct ExecutorRegistry {}

impl Actor for ExecutorRegistry {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("ExecutorRegistry started");
    }
}
