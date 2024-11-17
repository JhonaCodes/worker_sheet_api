use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn index_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}
