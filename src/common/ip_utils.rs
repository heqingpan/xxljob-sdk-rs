use std::net::Ipv4Addr;
use tokio::net::TcpListener;

pub fn get_local_ip() -> String {
    let mut ip_str = "127.0.0.1".to_string();
    let interfaces = if let Ok(v) = if_addrs::get_if_addrs() {
        v
    } else {
        return ip_str;
    };
    for interface in interfaces {
        if interface.is_loopback() || interface.is_link_local() {
            continue;
        }
        let ip = interface.ip();
        ip_str = ip.to_string();
        if ip.is_ipv4() {
            break;
        }
    }
    ip_str
}

pub async fn async_get_available_port(start_port: u16) -> u16 {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    for port in start_port..65535 {
        if async_check_port_available(ip, port).await {
            return port;
        }
    }
    0u16
}

pub async fn async_check_port_available(ip: Ipv4Addr, port: u16) -> bool {
    TcpListener::bind((ip, port)).await.is_ok()
}

pub fn get_available_port(start_port: u16) -> u16 {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    for port in start_port..65535 {
        if check_port_available(ip, port) {
            return port;
        }
    }
    0u16
}

pub fn check_port_available(ip: Ipv4Addr, port: u16) -> bool {
    // 尝试绑定到指定的端口，如果成功则说明端口未被占用
    match std::net::TcpListener::bind((ip, port)) {
        Ok(listener) => {
            if let Ok(v) = listener.local_addr() {
                v.port() == port
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
