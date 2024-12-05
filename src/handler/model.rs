use actix_web::HttpResponse;
use serde::Serialize;

// Estructuras base para respuestas
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    title: String,
    message: Option<T>
}

impl<T> ApiResponse<T> {
    pub fn new(title: &str, message: Option<T>) -> Self {
        Self {
            title: title.to_string(),
            message,
        }
    }
}

