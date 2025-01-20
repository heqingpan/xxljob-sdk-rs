use async_trait::async_trait;
use std::sync::Arc;
use xxljob_sdk_rs::client::builder::XxlClientBuilder;
use xxljob_sdk_rs::common::model::handler::{
    AsyncJobHandler, JobContext, JobHandler, SyncJobHandler,
};

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
        for i in 0..10 {
            log::info!(
                "async|test job do something... ; log_id:{}, step:{}",
                &context.log_id,
                i
            );
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        log::info!(
            "async|DemoJobHandler job process done; job_id:{}, log_id:{}",
            &context.job_id,
            &context.log_id
        );
        Ok(context)
    }
}

impl SyncJobHandler for DemoJobHandler {
    fn process(&self, context: JobContext) -> anyhow::Result<JobContext> {
        log::info!(
            "sync|DemoJobHandler job info; job_id:{}, log_id:{}, job_param:{:?}",
            &context.job_id,
            &context.log_id,
            &context.job_param
        );
        for i in 0..15 {
            log::info!(
                "sync|test job do something... ; log_id:{}, step:{}",
                &context.log_id,
                i
            );
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        log::info!(
            "sync|DemoJobHandler job process done; job_id:{}, log_id:{}",
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
        let client = XxlClientBuilder::new("http://127.0.0.1:8080/xxl-job-admin".to_string())
            .set_access_token("default_token".to_string())
            .set_log_path("xxl-rs-logs".to_string())
            .set_app_name("xxl-job-executor-sample".to_string())
            .build()?;
        client.register(
            Arc::new("demoJobHandler".to_owned()),
            //JobHandler::Async(Arc::new(DemoJobHandler {})),
            JobHandler::Sync(Arc::new(DemoJobHandler {})),
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
