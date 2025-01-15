use crate::common::share_data::ShareData;
use crate::server::xxlapi;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{middleware, web, App, HttpServer};
use std::sync::Arc;

pub fn api_config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/xxlapi")
            .service(web::resource("/beat").route(web::post().to(xxlapi::beat)))
            .service(web::resource("/idleBeat").route(web::post().to(xxlapi::idle_beat)))
            .service(web::resource("/run").route(web::post().to(xxlapi::run)))
            .service(web::resource("/kill").route(web::post().to(xxlapi::kill)))
            .service(web::resource("/log").route(web::post().to(xxlapi::log))),
    );
}

pub async fn run_embed_web(share_data: Arc<ShareData>) -> anyhow::Result<()> {
    let http_console_addr = share_data.client_config.get_http_addr();
    log::info!("run embed server http addr:{}", &http_console_addr);
    let app_data = Data::new(share_data.clone());
    HttpServer::new(move || {
        let app_data = app_data.clone();
        App::new()
            .app_data(app_data)
            .wrap(middleware::Logger::default())
            .configure(api_config)
    })
    .workers(1)
    .bind(http_console_addr)?
    .run()
    .await
    .ok();
    Ok(())
}
