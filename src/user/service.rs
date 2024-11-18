use super::models::{UserModel, UpdateUser, UpdateUserNotifications, UpdateUserPassword, UpdateUserStatus, UserFilters};
use actix_web::{get, patch, post, put, web, Responder};
use actix_web::web::{Path, Query};

use uuid::Uuid;
use web::{
    Json,
    Data
};
use crate::model::AppState;
use crate::user::repository::UserRepository;


#[post("/user")]
pub async fn create_user(conn: Data<AppState>, body: Json<UserModel>) -> impl Responder {
    UserRepository::create_user(conn, body).await
}
#[get("/user/{user_id}")]
pub async fn get_users(conn: Data<AppState>, id: Path<Uuid>) -> impl Responder {
    UserRepository::get_by_id(conn, id.into_inner() ).await
}

#[get("/users")]
pub async fn list_users(
    conn: web::Data<AppState>,
    filters: Query<UserFilters>
) -> impl Responder {
    UserRepository::list_users(conn, filters.into_inner()).await
}

#[put("/user/{id}")]
pub async fn update_user(
    conn: web::Data<AppState>,
    id: Path<Uuid>,
    user: Json<UpdateUser>
) -> impl Responder {
    UserRepository::update_user(conn, id.into_inner(), user.into_inner()).await
}

#[patch("/user/{id}/status")]
pub async fn update_user_status(
    conn: Data<AppState>,
    id: Path<Uuid>,
    status: Json<UpdateUserStatus>
) -> impl Responder {
    UserRepository::update_user_status(conn, id.into_inner(), status.into_inner()).await
}

#[patch("/user/{id}/notifications")]
pub async fn update_user_notifications( conn: Data<AppState>, id: Path<Uuid>,  notifications: Json<UpdateUserNotifications>
) -> impl Responder {
    UserRepository::update_user_notifications(conn, id.into_inner(), notifications.into_inner()).await
}

#[patch("/user/{id}/password")]
pub async fn update_user_password(conn: Data<AppState>, id: Path<Uuid>, password: Json<UpdateUserPassword>) -> impl Responder {
    UserRepository::update_user_password(conn, id.into_inner(), password.into_inner()).await
}