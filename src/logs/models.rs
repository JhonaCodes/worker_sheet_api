use std::sync::Mutex;
use chrono::Utc;
use serde::Serialize;
use lazy_static::lazy_static;
use std::collections::VecDeque;

#[derive(Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub path: String,
    pub method: Option<String>,
    pub ip: Option<String>,
    pub status: Option<u16>,
}