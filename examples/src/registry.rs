use xxljob_sdk_rs::client::builder::ExecutorBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    log::info!("registry start");
    let client = ExecutorBuilder::new("http://127.0.0.1:8080/xxl-job-admin".to_string())
        .set_ip("127.0.0.1".to_string())
        .set_port(9990)
        .set_access_token("default_token".to_string())
        .set_log_path("xxl-rs-logs".to_string())
        .set_app_name("xxl-job-executor-sample".to_string())
        .build()?;
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
    Ok(())
}
