use async_trait::async_trait;
use std::sync::Arc;
use xxljob_sdk_rs::client::builder::ExecutorBuilder;
use xxljob_sdk_rs::common::model::handler::{JobContext, JobHandler};

pub struct DemoJobHandler;

#[async_trait]
impl JobHandler for DemoJobHandler {
    async fn process(&self, context: JobContext) -> anyhow::Result<JobContext> {
        log::info!(
            "DemoJobHandler job info; job_id:{}, log_id:{}, job_param:{:?}",
            &context.job_id,
            &context.log_id,
            &context.job_param
        );
        for i in 0..15 {
            log::info!(
                "test job do something... ; log_id:{}, step:{}",
                &context.log_id,
                i
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        log::info!(
            "DemoJobHandler job process done; job_id:{}, log_id:{}",
            &context.job_id,
            &context.log_id
        );
        Ok(context)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    log::info!("registry start");
    {
        let client = ExecutorBuilder::new("http://127.0.0.1:8080/xxl-job-admin".to_string())
            .set_ip("127.0.0.1".to_string())
            .set_port(9991)
            .set_access_token("default_token".to_string())
            .set_log_path("xxl-rs-logs".to_string())
            .set_app_name("xxl-job-executor-sample".to_string())
            .build()?;
        client.register(
            Arc::new("demoJobHandler".to_owned()),
            Arc::new(DemoJobHandler {}),
        )?;
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for event");
        //executor unregister
        client.stop().await?;
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(())
}
