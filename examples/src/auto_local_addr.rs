use xxljob_sdk_rs::common::ip_utils::{get_available_port, get_local_ip};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let local_ip = get_local_ip();
    println!("Local IP: {}", local_ip);
    let available_port = get_available_port(9900);
    println!("Available Port: {}", available_port);
    Ok(())
}
