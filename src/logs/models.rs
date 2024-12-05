use serde::Serialize;
use std::collections::{HashMap};

#[derive(Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub path: String,
    pub method: Option<String>,
    pub status: Option<u16>,
    pub ip: Option<String>,
    pub response_time_ms: Option<u128>,
    pub user_agent: Option<String>,
    pub error_detail: Option<String>,
    pub headers: Option<HashMap<String, String>>
}