use actix_cors::Cors;

use actix_web::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use actix_web::http::Method;
use actix_web::web;
use crate::auth::service::{basic_auth, jwt_profile_validate, refresh_token};
use crate::helper::service::health_revision;
use crate::r#static::service::{api_doc_page, index_page};
use crate::user::service::create_user;

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


pub fn config_crud_users(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/register").service(create_user));
}