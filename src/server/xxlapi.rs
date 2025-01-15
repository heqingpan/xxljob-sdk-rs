use crate::server::model::xxl_api_empty_success;
use actix_web::{HttpResponse, Responder};

pub(crate) async fn beat() -> impl Responder {
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn idle_beat() -> impl Responder {
    //todo check something
    HttpResponse::Ok().json(xxl_api_empty_success())
}

pub(crate) async fn run() -> impl Responder {
    //todo run job
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
