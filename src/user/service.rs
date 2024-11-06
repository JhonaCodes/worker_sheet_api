use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::db::DbPool;
use super::models::{Users, NewUser};
use super::repository;

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match web::block(move || repository::get_all_users(&mut conn)).await {
        Ok(users) => HttpResponse::Ok().json(users.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/users/{id}")]
pub async fn get_user(pool: web::Data<DbPool>, id: web::Path<String>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let user_id = id.into_inner();

    match web::block(move || repository::get_user_by_id(&mut conn, &user_id)).await {
        Ok(user) => HttpResponse::Ok().json(user.unwrap()),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match web::block(move || repository::create_user(&mut conn, new_user.into_inner())).await {
        Ok(user) => HttpResponse::Created().json(user.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
    user_data: web::Json<NewUser>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let user_id = id.into_inner();

    match web::block(move || repository::update_user(&mut conn, &user_id, user_data.into_inner())).await {
        Ok(user) => HttpResponse::Ok().json(user.unwrap()),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

#[delete("/users/{id}")]
pub async fn delete_user( pool: web::Data<DbPool>, id: web::Path<String>, ) -> impl Responder {
    
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let user_id = id.into_inner();

    match web::block(move || repository::delete_user(&mut conn, &user_id)).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish()
    }
}