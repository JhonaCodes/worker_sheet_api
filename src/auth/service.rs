use actix_web::{ post, web::Json, Responder};
use actix_web::web::Data;

use crate::auth::models::LoginProfileModel;
use crate::auth::repository::AuthRepository;
use crate::model::AppState;

#[post("/login")]
async fn basic_auth(state: Data<AppState>, credentials: Json<LoginProfileModel>) -> impl Responder {
    AuthRepository::basic_auth(state, credentials).await
}


#[post("/jwt_validate")]
async fn jwt_profile_validate() -> impl Responder{
    AuthRepository::check_jwt().await
}

#[post("/refresh")]
async fn refresh_token(state: Data<AppState>, credentials: Json<LoginProfileModel>)-> impl Responder {
    AuthRepository::basic_auth(state, credentials).await
}