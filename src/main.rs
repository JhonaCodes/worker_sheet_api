use axum::{
    routing::get,
    Router,
    response::Json,
    extract::State,
};
use serde::{Serialize, Deserialize};

use std::sync::Arc;
use axum::extract::Path;
use rust_decimal::Decimal;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Worker {
    id: i32,
    name: String,
    position: String,
    department: Option<String>,
    salary: Decimal,
    created_at: Option<OffsetDateTime>,
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

async fn health_check() -> &'static str {
    "API Funcionando!"
}

async fn get_workers(
    State(state): State<Arc<AppState>>
) -> Json<Vec<Worker>> {
    let workers = sqlx::query_as::<_, Worker>(
        "SELECT id, name, position, department, salary, created_at FROM workers"
    )
        .fetch_all(&state.pool)
        .await
        .unwrap_or(vec![]);

    Json(workers)
}

// Endpoint para obtener un trabajador por ID
async fn get_worker_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Json<Option<Worker>> {
    let worker = sqlx::query_as::<_, Worker>(
        "SELECT id, name, position, department, salary, created_at FROM workers WHERE id = $1"
    )
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None);

    Json(worker)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar la conexión a la base de datos
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@postgres-db:5432/midb".to_string());


    // Crear el pool de conexiones
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*database_url)
        .await?;

    // Crear el estado compartido
    let state = Arc::new(AppState { pool });

    // Configurar el router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/workers", get(get_workers))
        .route("/workers/:id", get(get_worker_by_id))
        .with_state(state);

    println!("Servidor corriendo en http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}