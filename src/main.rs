mod auth;
mod db;
mod env;
pub mod schema;
mod r#static;
mod user;

use crate::env::models::AppConfig;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

use std::io::Result;

use crate::auth::service::user_data;
use crate::db::establish_connection_pool;
use crate::r#static::service::index_page;
// use crate::user::service::{create_user, delete_user, get_user, get_users, update_user};

#[actix_web::main]
async fn main() -> Result<()> {
    // Cargar el entorno una sola vez al inicio
    if dotenvy::from_filename("dev.env").is_err() {
        dotenvy::from_filename(".env").ok();
    }

    // Initialization for logger
    env_logger::init();
    dotenv().ok();

    // Contain a local environments and servers data.
    let app_config = AppConfig::init_config();

    // Initialize database connection pool
    let pool = establish_connection_pool();

    // Server configuration
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(index_page)
            .service(user_data)
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
