use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::sync::Mutex;
use chrono::Utc;
use crate::logs::models::LogEntry;


// Mantener los últimos 1000 logs en memoria
lazy_static! {
    static ref LOG_HISTORY: Mutex<VecDeque<LogEntry>> = Mutex::new(VecDeque::with_capacity(1000));
}

pub fn add_log_entry(
    level: &str,
    message: &str,
    path: &str,
    method: Option<String>,
    status: Option<u16>,
    ip: Option<String>,
) {
    let entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        level: level.to_string(),
        message: message.to_string(),
        path: path.to_string(),
        method,
        ip: ip.clone(),
        status,
    };

    let mut history = LOG_HISTORY.lock().unwrap();
    if history.len() >= 1000 {
        history.pop_front(); // Elimina el log más antiguo si alcanzamos el límite
    }
    history.push_back(entry);
}

pub fn get_logs(limit: Option<usize>) -> Vec<LogEntry> {
    let history = LOG_HISTORY.lock().unwrap();
    let limit = limit.unwrap_or(100);
    history.iter()
        .rev() // Más recientes primero
        .take(limit)
        .cloned()
        .collect()
}