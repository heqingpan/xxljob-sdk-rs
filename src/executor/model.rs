use crate::common::model::admin_request::CallbackParam;
use actix::Message;

#[derive(Message, Clone, Debug)]
#[rtype(result = "anyhow::Result<ServerAccessActorResult>")]
pub enum ServerAccessActorReq {
    Stop,
    CallBack(Vec<CallbackParam>),
}

pub enum ServerAccessActorResult {
    None,
}
