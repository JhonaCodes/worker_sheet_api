use sqlx::{Pool, Postgres};

pub struct AppState {
    pub(crate) db: Pool<Postgres>,
}