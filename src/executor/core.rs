#![allow(unused_variables, dead_code)]
use crate::common::client_config::ClientConfig;
use crate::common::model::enum_type::ExecutorBlockStrategy;
use crate::common::model::handler::{JobContext, JobHandler, JobHandlerRunParam, JobHandlerValue};
use crate::common::model::FAIL_CODE;
use crate::executor::admin_server::{callback, ServerAccessActor};
use crate::executor::model::{ExecutorActorReq, ExecutorActorResult};
use actix::prelude::*;
use bean_factory::{bean, BeanFactory, FactoryData, Inject};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Default)]
#[bean(inject)]
pub struct ExecutorActor {
    client_config: Arc<ClientConfig>,
    job_handler_map: HashMap<Arc<String>, JobHandlerValue>,
    server_access_actor: Option<Addr<ServerAccessActor>>,
}

impl ExecutorActor {
    pub fn new(client_config: Arc<ClientConfig>) -> Self {
        Self {
            client_config,
            job_handler_map: HashMap::new(),
            server_access_actor: None,
        }
    }

    fn register_job_handler(&mut self, job_handler: JobHandlerValue) {
        self.job_handler_map
            .insert(job_handler.name.clone(), job_handler);
    }

    fn run_job(
        &mut self,
        job_name: Arc<String>,
        job_context: JobContext,
        ctx: &mut Context<Self>,
    ) -> anyhow::Result<ExecutorActorResult> {
        let run_param = if let Some(handler_value) = self.job_handler_map.get_mut(&job_name) {
            if handler_value.is_running {
                match &job_context.block_strategy {
                    ExecutorBlockStrategy::SerialExecution => {
                        //如果超过排队上限会移除前面任务
                        if let Some(old_job) = handler_value.push_block_job(job_context) {
                            old_job.callback_failed();
                        }
                        return Ok(ExecutorActorResult::Ok);
                    }
                    ExecutorBlockStrategy::DiscardLater => {
                        job_context.callback_failed_with_info(
                            format!(
                                "Discard the job; job_id:{}, log_id:{}",
                                job_context.job_id, job_context.log_id
                            ),
                            FAIL_CODE,
                        );
                        return Ok(ExecutorActorResult::Discard);
                    }
                    ExecutorBlockStrategy::CoverEarly | ExecutorBlockStrategy::Other => {}
                }
            }
            handler_value.is_running = true;
            handler_value.last_run_id = job_context.log_id;
            handler_value.build_run_param()
        } else {
            return Err(anyhow::anyhow!(
                "No handler registered for job:{}",
                job_name.as_str()
            ));
        };
        self.do_run_job(job_context, run_param, ctx);
        Ok(ExecutorActorResult::Ok)
    }

    fn do_run_job(
        &mut self,
        job_context: JobContext,
        job_handler_param: JobHandlerRunParam,
        ctx: &mut Context<Self>,
    ) {
        let job_handler = job_handler_param.handler.clone();
        let job_name = job_handler_param.name.clone();
        let log_id = job_context.log_id.to_owned();

        async move {
            match job_handler {
                JobHandler::Async(handler) => {
                    (handler.process(job_context).await, job_name, log_id)
                }
                JobHandler::Sync(handler) => {
                    let (tx, rx) = tokio::sync::oneshot::channel();
                    std::thread::spawn(move || {
                        let ctx = handler.process(job_context);
                        tx.send(ctx).ok();
                    });
                    let res = match rx.await {
                        Ok(v) => v,
                        Err(e) => Err(anyhow::anyhow!(e)),
                    };
                    (res, job_name, log_id)
                }
            }
        }
        .into_actor(self)
        .map(|(r, job_name, log_id), act, ctx| {
            match r {
                Ok(job) => {
                    job.callback_success();
                }
                Err(err) => {
                    //失败时取不到job对象，通过job_id反馈结果
                    if let Some(addr) = act.server_access_actor.as_ref() {
                        callback(addr, log_id.to_owned(), FAIL_CODE, Some(err.to_string()));
                    }
                }
            };
            if let Some(value) = act.job_handler_map.get_mut(&job_name) {
                if value.last_run_id == log_id {
                    value.is_running = false;
                    value.last_run_id = 0;
                }
                if !value.block_jobs.is_empty() {
                    act.run_next_block_job(job_name, ctx);
                }
            };
        })
        .spawn(ctx);
    }

    fn run_next_block_job(&mut self, job_name: Arc<String>, ctx: &mut Context<Self>) {
        let (job, run_param) = if let Some(value) = self.job_handler_map.get_mut(&job_name) {
            if let Some(job) = value.pop_block_job() {
                value.is_running = true;
                value.last_run_id = job.log_id;
                (job, value.build_run_param())
            } else {
                return;
            }
        } else {
            return;
        };
        self.do_run_job(job, run_param, ctx);
    }
}

impl Actor for ExecutorActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Executor actor started");
    }
}

impl Inject for ExecutorActor {
    type Context = Context<Self>;

    fn inject(
        &mut self,
        factory_data: FactoryData,
        _factory: BeanFactory,
        _ctx: &mut Self::Context,
    ) {
        self.server_access_actor = factory_data.get_actor();
    }
}

impl Supervised for ExecutorActor {}

impl Handler<ExecutorActorReq> for ExecutorActor {
    type Result = anyhow::Result<ExecutorActorResult>;

    fn handle(&mut self, msg: ExecutorActorReq, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ExecutorActorReq::Register(job_handler_value) => {
                self.register_job_handler(job_handler_value);
                Ok(ExecutorActorResult::Ok)
            }
            ExecutorActorReq::RunJob {
                job_name,
                job_content,
            } => self.run_job(job_name, job_content, ctx),
        }
    }
}
