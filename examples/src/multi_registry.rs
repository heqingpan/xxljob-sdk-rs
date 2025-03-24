use async_trait::async_trait;
use std::sync::Arc;
use xxljob_sdk_rs::{get_last_xxl_client, XxlClient, XxlClientBuilder};
use xxljob_sdk_rs::{AsyncJobHandler, JobContext, JobHandler, SyncJobHandler};

pub struct DemoJobHandler;

#[async_trait]
impl AsyncJobHandler for DemoJobHandler {
    async fn process(&self, context: JobContext) -> anyhow::Result<JobContext> {
        log::info!(
            "async|DemoJobHandler job info; job_id:{}, log_id:{}, job_param:{:?}",
            &context.job_id,
            &context.log_id,
            &context.job_param
        );
        let sleep_count = context
            .job_param
            .clone()
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap_or(1);
        tokio::time::sleep(std::time::Duration::from_millis(sleep_count * 100)).await;
        log::info!(
            "async|DemoJobHandler job process done; job_id:{}, log_id:{}",
            &context.job_id,
            &context.log_id
        );
        Ok(context)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //std::env::set_var("RUST_LOG", "INFO");
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();
    log::info!("registry start");
    let admin_url =
        std::env::var("ADMIN_URL").unwrap_or("http://127.0.0.1:8081/xxl-job-admin".to_string());
    let client_count = std::env::var("CLIENT_COUNT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(20);
    let handler_count = std::env::var("HANDLER_COUNT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(2000);
    let mut clients = vec![];
    for i in 0..client_count {
        let client = XxlClientBuilder::new(admin_url.clone())
            .set_access_token("default_token".to_string())
            .set_log_path("xxl-rs-logs".to_string())
            .set_app_name("xxl-job-executor-sample".to_string())
            .build()?;
        clients.push(client);
    }
    for client in &clients {
        for i in 0..handler_count {
            client.register(
                Arc::new(format!("demoJobHandler{:04}", i)),
                JobHandler::Async(Arc::new(DemoJobHandler {})),
            )?;
        }
    }
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
    Ok(())
}
