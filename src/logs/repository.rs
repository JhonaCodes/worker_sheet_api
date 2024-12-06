use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Write};
use std::path::Path;
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
    response_time_ms: Option<u128>,
    user_agent: Option<String>,
    error_detail: Option<String>,
    headers: Option<HashMap<String, String>>
) {
    let entry = LogEntry {
        timestamp: Utc::now().to_rfc3339(),
        level: level.to_string(),
        message: message.to_string(),
        path: path.to_string(),
        method,
        status,
        ip,
        response_time_ms,
        user_agent,
        error_detail,
        headers
    };



    let mut history = LOG_HISTORY.lock().unwrap();
    if history.len() >= 1000 {
        history.pop_front();
    }

    history.push_back(entry.clone());

    // Guardar en archivo
    write_log_to_file(&entry);
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


pub fn write_log_to_file(log_entry: &LogEntry) {
    let today = Utc::now().date_naive();
    let filename = format!("logs/worker_sheet_{}.log", today);

    std::fs::create_dir_all("logs").unwrap_or_default();

    let log_json = serde_json::to_string(&log_entry).unwrap();

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)
    {
        // Usar write_all en lugar de writeln!
        file.write_all(log_json.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();  // Agregar salto de línea
    }

    rotate_if_needed(&filename);
}

fn rotate_if_needed(filename: &str) {
    let path = Path::new(filename);
    if let Ok(_) = std::fs::metadata(path) {
        let line_count = std::io::BufReader::new(File::open(path).unwrap())
            .lines()
            .count();

        if line_count > 2000 {
            let new_filename = format!("{}.old", filename);
            std::fs::rename(filename, new_filename).unwrap_or_default();
        }
    }
}