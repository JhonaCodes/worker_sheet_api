mod auth;
mod db;
mod env;
mod r#static;
mod user;
mod model;
mod activities;
mod helper;
mod participants;

use crate::env::models::AppConfig;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use std::io::Result;
use actix_cors::Cors;
use actix_web::web::{service, Data};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::postgres::PgPoolOptions;
use crate::activities::service::{add_photo, create_activity, delete_activity, delete_photo, get_activity, get_activity_list_by_user_id, get_photos, list_activities, update_activity, update_activity_status};
use crate::auth::env::validate_jwt;
use crate::auth::service::{basic_auth, jwt_profile_validate, refresh_token};
use crate::db::url_database;
use crate::model::AppState;
use crate::participants::service::{create_participant, get_activities_by_participant_id, get_participants};
use crate::r#static::service::index_page;
use crate::user::service::{create_user, get_users, update_user, update_user_notifications, update_user_password, update_user_status};

#[actix_web::main]
async fn main() -> Result<()> {

    // Cargar el entorno una sola vez al inicio
    if dotenvy::from_filename("dev.env").is_err() {
        dotenvy::from_filename(".env").ok();
    }

    // Conexion maxima permitida y demás configruaciones, se debe separar.
    let pool = PgPoolOptions::new()
        .max_connections(700)
        .connect(&url_database())
        .await
        .expect("Error building a connection pool");


    let jwt_bearer_middleware = HttpAuthentication::bearer(validate_jwt);

    
    // Rules for initialize app state.
    let app_state = Data::new(AppState { db: pool.clone() });
    
    // Initialization for logger
    env_logger::init();
    dotenv().ok();

    // Contain a local environments and servers data.
    let app_config = AppConfig::init_config();

    // Server configuration
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()// Todo: We need improve cors
                    .allow_any_method()// Todo: We need restricted methods
                    .allow_any_header()// Todo: We need to restricted header
            )
            .wrap(actix_web::middleware::Logger::default())
            .service(index_page)
            .service(web::scope("/register").service(create_user))
            .service(web::scope("/auth")
                         .service(jwt_profile_validate)
                         .service(refresh_token)
                         .service(basic_auth),
            )
            .service(web::scope("/v1")
                .wrap(jwt_bearer_middleware.clone())
                .service(get_users)
                .service(update_user)
                .service(update_user_status)
                .service(update_user_notifications)
                .service(update_user_password)
                .service(create_activity)
                .service(get_activity)
                .service(delete_activity)
                .service(list_activities)
                .service(update_activity)
                .service(update_activity_status)
                .service(add_photo)
                .service(get_photos)
                .service(delete_photo)
                .service(create_participant)
                .service(get_participants)
                .service(get_activities_by_participant_id)
                .service(get_activity_list_by_user_id)
            )
            

    });

    println!(
        "Server running {}:{}",
        app_config.server.host, app_config.server.port
    );

    // Server initialization
    server
        .bind((app_config.server.host, app_config.server.port))?
        .run()
        .await
}


