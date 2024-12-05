mod auth;
mod db;
mod env;
mod r#static;
mod user;
mod model;
mod activities;
mod helper;
mod participants;
mod service;
mod logs;

use crate::env::models::AppConfig;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use std::io::Result;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{Data};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::postgres::PgPoolOptions;
use crate::activities::service::{add_photo, create_activity, delete_activity, delete_photo, get_activity, get_activity_list_by_user_id, get_photos, list_activities, update_activity, update_activity_status};
use crate::auth::env::{validate_jwt, validate_jwt_admin};
use crate::db::url_database;
use crate::logs::middlewares;
use crate::logs::service::{config_log, get_system_logs};
use crate::model::AppState;
use crate::participants::service::{create_participant, get_activities_by_participant_id, get_participants_by_activity_id};
use crate::service::{config_auth, config_cors, config_crud_users, config_server_state, config_static_pages};
use crate::user::service::{ delete_user, get_users, update_user, update_user_notifications, update_user_password, update_user_status};

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
    let jwt_bearer_admin = HttpAuthentication::bearer(validate_jwt_admin);

    
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
           // .wrap(Logger::default())
            .wrap(middlewares::RequestLogger)
            // .configure(config_log)
            .wrap( config_cors() )
            .app_data(app_state.clone())
            .configure(config_server_state)
            .configure(config_static_pages)
            .configure(config_auth)
            .configure(config_crud_users)
            .service(web::scope("/v1")
                .service(web::scope("/admin").wrap(jwt_bearer_admin.clone()).service(get_system_logs))
                //.service(Files::new("/uploads", "/app/uploads"))
                .wrap(jwt_bearer_middleware.clone())
                .service(get_users)
                .service(update_user)
                .service(update_user_status)
                .service(update_user_notifications)
                .service(update_user_password)
                .service(delete_user)
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
                .service(get_participants_by_activity_id)
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