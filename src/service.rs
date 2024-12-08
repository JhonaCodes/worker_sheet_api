use actix_cors::Cors;
use actix_files::Files;
use actix_web::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use actix_web::http::Method;
use actix_web::web;

use crate::activities::service::{add_photo, create_activity, delete_activity, delete_photo, get_activity, get_activity_list_by_user_id, get_photos, list_activities, update_activity, update_activity_status};
use crate::auth::service::{basic_auth, jwt_profile_validate, refresh_token};
use crate::helper::service::health_revision;

use crate::participants::service::{create_participant, get_activities_by_participant_id, get_participants_by_activity_id};
use crate::r#static::service::{api_doc_page, index_page};
use crate::user::service::{create_user, delete_user, get_users, update_user, update_user_notifications, update_user_password, update_user_status};

pub fn config_cors() -> Cors {

    let allowed_methods = vec![Method::GET,Method::POST,Method:: PUT,Method:: PATCH,Method:: DELETE, Method::OPTIONS];
    let allowed_headers = vec![ACCEPT, CONTENT_TYPE, AUTHORIZATION];

    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://workersheet.com")
        .allowed_methods(allowed_methods)
        .allowed_headers(allowed_headers)
        .supports_credentials()
        .max_age(3600)
}

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(jwt_profile_validate)
            .service(refresh_token)
            .service(basic_auth)
    );
}

pub fn config_static_pages(cfg: &mut web::ServiceConfig) {
    cfg.service(index_page)
        .service(api_doc_page);
}

pub fn config_server_state(cfg: &mut web::ServiceConfig) {
    cfg.service(health_revision);
}


pub fn config_signup_users(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/register").service(create_user));
}

pub fn config_crud_users(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
        .service(update_user)
        .service(update_user_status)
        .service(update_user_notifications)
        .service(update_user_password)
        .service(delete_user);
}

pub fn config_upload_files(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/uploads", "/app/uploads"))
        .service(get_photos)
        .service(delete_photo)
        .service(add_photo);
}


pub fn config_crud_activities(cfg: &mut web::ServiceConfig) {
    cfg.service(create_activity)
        .service(get_activity)
        .service(delete_activity)
        .service(list_activities)
        .service(update_activity)
        .service(update_activity_status)
        .service(get_activity_list_by_user_id);
}

pub fn config_participants(cfg: &mut web::ServiceConfig) {
    cfg.service(create_participant)
        .service(get_participants_by_activity_id)
        .service(get_activities_by_participant_id);
}