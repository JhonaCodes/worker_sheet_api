use super::models::{NewUser};
use actix_web::{get, post,  web, Responder};
use actix_web::web::Path;

use uuid::Uuid;
use web::{
    Json,
    Data
};
use crate::model::AppState;
use crate::user::repository::UserRepository;


#[post("/user")]
pub async fn create_user(conn: Data<AppState>, new_user: Json<NewUser>) -> impl Responder {
    UserRepository::create_user(conn, new_user.into_inner()).await
}
#[get("/user/{user_id}")]
pub async fn get_users(conn: Data<AppState>, id: Path<Uuid>) -> impl Responder {
    UserRepository::get_by_id(conn, id.into_inner() ).await
}
