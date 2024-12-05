use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use crate::logs::repository::get_logs;

#[derive(Deserialize)]
pub struct LogQuery {
    limit: Option<usize>,
}

#[get("/logs")]
async fn get_system_logs(query: web::Query<LogQuery>) -> HttpResponse {
    let logs = get_logs(query.limit);
    HttpResponse::Ok().json(logs)
}

pub fn config_log(cfg: &mut web::ServiceConfig) {
    cfg.service(get_system_logs);
}