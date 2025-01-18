use crate::common::model::admin_request::CallbackParam;
use crate::common::model::handler::{JobContext, JobHandlerValue};
use actix::Message;
use std::sync::Arc;

#[derive(Message, Clone, Debug)]
#[rtype(result = "anyhow::Result<ServerAccessActorResult>")]
pub enum ServerAccessActorReq {
    Stop,
    CallBack(Vec<CallbackParam>),
}

pub enum ServerAccessActorResult {
    None,
}

#[derive(Message, Clone)]
#[rtype(result = "anyhow::Result<ExecutorActorResult>")]
pub enum ExecutorActorReq {
    Register(JobHandlerValue),
    RunJob {
        job_name: Arc<String>,
        job_content: JobContext,
    },
}

pub enum ExecutorActorResult {
    Ok,
    NotFoundJob,
    Discard,
}
