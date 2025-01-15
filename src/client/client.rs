use crate::common::client_config::ClientConfig;
use crate::common::share_data::ShareData;
use crate::server::web_server::run_embed_web;
use std::sync::Arc;

pub struct XxlClient {
    pub(crate) client_config: Arc<ClientConfig>,
    pub(crate) share_data: Arc<ShareData>,
    is_running: bool,
}

impl XxlClient {
    pub(crate) fn new(client_config: Arc<ClientConfig>, share_data: Arc<ShareData>) -> XxlClient {
        Self {
            client_config,
            share_data,
            is_running: false,
        }
    }
    pub fn run(&mut self) {
        if self.is_running {
            return;
        }
        let share_data = self.share_data.clone();
        std::thread::spawn(move || {
            actix_rt::System::with_tokio_rt(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
            })
            .block_on(run_embed_web(share_data));
        });
        self.is_running = true;
        //todo registry executor
    }
}
