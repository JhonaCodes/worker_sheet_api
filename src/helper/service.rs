use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
pub async fn health_revision() -> impl Responder {
    // Obtener uptime del sistema
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Estado de la aplicación
    let app_status = "healthy";

    // Información del sistema
    let system_info = json!({
        "status": app_status,
        "timestamp": since_epoch,
        "version": env!("CARGO_PKG_VERSION"),
        "environment": std::env::var("APP_ENV").unwrap_or_else(|_| "production".to_string()),
    });

    // Construir la respuesta
    HttpResponse::Ok().json(system_info)
}