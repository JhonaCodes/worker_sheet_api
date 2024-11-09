use std::env;

pub fn url_database() -> String{
    return env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}