use crate::common::share_data::ShareData;
use std::sync::Arc;

pub struct XxlClient {
    pub(crate) share_data: Arc<ShareData>,
    is_running: bool,
}

impl XxlClient {
    pub(crate) fn new(share_data: Arc<ShareData>) -> XxlClient {
        Self {
            share_data,
            is_running: false,
        }
    }
}
