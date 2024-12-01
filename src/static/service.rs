use actix_web::{get, HttpResponse};

#[get("/")]
pub async fn index_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html"))
}

#[get("/api/documentation")]
pub async fn api_doc_page() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/api_doc.html"))
}
