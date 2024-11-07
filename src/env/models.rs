use serde::Deserialize;

use ::std::env;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub user: String,
    pub password: String,
    pub port: u16,
    pub max_connections: u16,
    pub min_connections: u16,
    pub pool_size: u16,
    pub database_name: String,
}

impl DatabaseConfig {
    pub fn get_env() -> DatabaseConfig {
        DatabaseConfig {
            url: env::var("DATABASE_URL").unwrap(),
            user: env::var("DATABASE_USER").unwrap(),
            password: env::var("DATABASE_PASSWORD").unwrap(),
            port: env::var("DATABASE_PORT").unwrap().parse().unwrap(),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap()
                .parse()
                .unwrap(),
            min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap()
                .parse()
                .unwrap(),
            pool_size: env::var("DATABASE_POOL_SIZE").unwrap().parse().unwrap(),
            database_name: env::var("DATABASE_NAME").unwrap(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

impl ServerConfig {
    pub fn get_env() -> ServerConfig {
        ServerConfig {
            port: env::var("SERVER_PORT").unwrap().parse().unwrap(),
            host: env::var("SERVER_HOST").unwrap().parse().unwrap(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub is_debug_mode: bool,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl AppConfig {
    /// Initialization of general configuration
    pub fn init_config() -> AppConfig {
        AppConfig {
            is_debug_mode: env::var("DEBUG_MODE").unwrap().parse().unwrap(),
            database: DatabaseConfig::get_env(),
            server: ServerConfig::get_env(),
        }
    }
}
