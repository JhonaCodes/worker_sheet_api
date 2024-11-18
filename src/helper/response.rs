use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;
use sqlx::postgres::PgTypeKind::Simple;

use crate::helper::models::MessageResponse;

pub fn string(text:&str) -> String{
    return String::from(text);
}

pub fn susses(message: &str ) -> HttpResponse{
    return HttpResponse::Ok().json(MessageResponse::susses(string(message)));
}

pub fn susses_json<T: Serialize>(body: T) -> HttpResponse{
    return HttpResponse::Ok().json(body)
}

pub fn un_susses( message: &str) -> HttpResponse{
    return HttpResponse::Forbidden().json(MessageResponse::error( string(message)));
}

pub fn un_susses_json<T: Serialize>(body: T) -> HttpResponse{
    return HttpResponse::Forbidden().json(body);
}

pub fn some_string(text:&str) -> Option<String> {
   return Some(string(text));
}