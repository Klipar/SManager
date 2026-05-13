use std::sync::{Arc};
use std::sync::atomic::AtomicBool;

pub struct ConnectionContext {
    pub id: Option<i32>,
    pub conn_id: u64,
    pub ip: String,
    pub user_id: Option<i32>,
    pub is_admin: bool,
    pub is_closing: Arc<AtomicBool>,
}

impl ConnectionContext {
    pub fn new(ip: String, conn_id: u64) -> Self {
        Self {
            id: None,
            conn_id,
            ip,
            user_id: None,
            is_admin: false,
            is_closing: Arc::new(AtomicBool::new(false)),
        }
    }
}