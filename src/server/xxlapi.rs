use crate::common::model::api_model::{JobIdleBeatParam, JobRunParam};
use crate::common::model::handler::JobContext;
use crate::common::model::{xxl_api_empty_success, XxlApiResult};
use crate::common::share_data::ShareData;
use crate::executor::model::{ExecutorActorReq, ExecutorActorResult};
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub(crate) async fn beat() -> impl Responder {
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn idle_beat(
    share_data: Data<Arc<ShareData>>,
    web::Json(param): web::Json<JobIdleBeatParam>,
) -> impl Responder {
    log::info!("idle_beat api param:{:?}", &param);
    if let Ok(Ok(ExecutorActorResult::Ok)) = share_data
        .executor_actor
        .send(ExecutorActorReq::IdleBeat {
            job_id: param.job_id,
        })
        .await
    {
        HttpResponse::Ok().json(xxl_api_empty_success())
    } else {
        HttpResponse::Ok().json(XxlApiResult::<()>::fail(Some(
            "job is running or has trigger queue.".to_string(),
        )))
    }
}

pub(crate) async fn run(
    share_data: Data<Arc<ShareData>>,
    web::Json(run_param): web::Json<JobRunParam>,
) -> impl Responder {
    log::info!("run api param:{:?}", &run_param);
    let job_name = run_param.executor_handler.clone().unwrap_or_default();
    if job_name.is_empty() {
        return HttpResponse::Ok().json(XxlApiResult::<()>::fail(Some(format!(
            "executor_handler is empty,log_id:{}",
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

pub(crate) async fn kill(web::Json(param): web::Json<JobIdleBeatParam>) -> impl Responder {
    log::info!("kill api param:{:?}", &param);
    //todo kill job
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn log() -> impl Responder {
    //todo load log
    HttpResponse::Ok().json(xxl_api_empty_success())
}
