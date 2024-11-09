mod auth;
mod db;
mod env;
mod r#static;
mod user;
mod model;

use crate::env::models::AppConfig;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

use std::io::Result;
use actix_web::web::Data;
use sqlx::postgres::PgPoolOptions;
use crate::auth::service::user_data;
use crate::db::url_database;
use crate::model::AppState;
use crate::r#static::service::index_page;
use crate::user::service::{create_user, get_users};

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


