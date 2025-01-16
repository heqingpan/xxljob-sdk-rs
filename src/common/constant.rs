use std::sync::Arc;

lazy_static::lazy_static! {
    pub static ref EXECUTOR: Arc<String> =  Arc::new("EXECUTOR".to_string());
}
