use crate::common::model::admin_request::CallbackParam;
use crate::common::model::api_model::JobRunParam;
use crate::common::model::handler::JobContext;
use crate::common::model::{xxl_api_empty_success, XxlApiResult, SUCCESS_CODE};
use crate::common::now_millis_i64;
use crate::common::share_data::ShareData;
use crate::executor::model::{ExecutorActorReq, ServerAccessActorReq};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub(crate) async fn beat() -> impl Responder {
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn idle_beat() -> impl Responder {
    //todo check something
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn run(
    share_data: Data<Arc<ShareData>>,
    web::Json(run_param): web::Json<JobRunParam>,
) -> impl Responder {
    log::info!("run api param:{:?}", &run_param);
    let job_name = run_param.executor_handler.clone().unwrap_or_default();
    if job_name.is_empty() {
        return HttpResponse::Ok().json(XxlApiResult::<()>::fail(Some(format!(
            "executor_handler is emtpy,log_id:{}",
            run_param.log_id
        ))));
    };
    let job_content = JobContext::new(run_param, share_data.as_ref().clone());
    share_data.executor_actor.do_send(ExecutorActorReq::RunJob {
        job_name,
        job_content,
    });
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn kill() -> impl Responder {
    //todo kill job
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn log() -> impl Responder {
    //todo load log
    HttpResponse::Ok().json(xxl_api_empty_success())
}
