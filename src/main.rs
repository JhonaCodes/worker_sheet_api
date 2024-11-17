mod auth;
mod db;
mod env;
mod r#static;
mod user;
mod model;
mod activities;

use crate::env::models::AppConfig;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

use std::io::Result;
use actix_web::web::Data;
use sqlx::postgres::PgPoolOptions;
use crate::activities::service::{add_photo, create_activity, delete_activity, delete_photo, get_activity, get_photos, list_activities, update_activity, update_activity_status};
use crate::auth::service::user_data;
use crate::db::url_database;
use crate::model::AppState;
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
            .wrap(actix_web::middleware::Logger::default())
            .service(index_page)
            .service(user_data)
            .service(create_user)
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


