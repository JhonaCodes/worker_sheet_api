# Guía de Implementación de Actix-Web con Diesel ORM

## Índice
1. [Introducción](#introducción)
2. [Configuración del Proyecto](#configuración-del-proyecto)
3. [Estructura y Componentes](#estructura-y-componentes)
4. [Implementación Detallada](#implementación-detallada)
5. [Pool de Conexiones](#pool-de-conexiones)
6. [Servicios REST](#servicios-rest)
7. [Configuración del Servidor](#configuración-del-servidor)

## Introducción

Esta guía explica cómo integrar Actix-Web con Diesel ORM, implementando una API REST con pool de conexiones y manejo asíncrono. La arquitectura está organizada por features y utiliza las mejores prácticas de Rust para web services.

## Configuración del Proyecto

### Cargo.toml
```toml
[dependencies]
actix-web = "4.0"
diesel = { version = "2.1.0", features = ["postgres", "r2d2"] }
r2d2 = "0.8"
dotenvy = "0.15"
serde = { version = "1.0", features = ["derive"] }
env_logger = "0.9"
```

### Estructura de Directorios
```
src/
├── main.rs           # Punto de entrada y configuración del servidor
├── db.rs             # Configuración de la base de datos y pool
├── env/
│   └── models.rs     # Configuración del entorno (AppConfig)
└── user/
    ├── models.rs     # Modelos de usuario
    ├── repository.rs # Operaciones de base de datos
    └── service.rs    # Endpoints de la API
```

## Estructura y Componentes

### Pool de Conexiones (db.rs)
```rust
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
```

El pool de conexiones es fundamental porque:
- Reutiliza conexiones en lugar de crear nuevas para cada request
- Maneja automáticamente la reconexión
- Limita el número máximo de conexiones simultáneas
- Es thread-safe y puede ser compartido entre workers

### Configuración del Entorno (env/models.rs)
```rust
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn init_config() -> Self {
        // Inicialización de configuración desde variables de entorno
        // ...
    }
}
```

## Implementación de Servicios REST

### Service Layer (user/service.rs)
Los servicios son endpoints HTTP que:
1. Reciben requests
2. Manejan la lógica de negocio
3. Interactúan con el repositorio
4. Devuelven respuestas HTTP apropiadas

```rust
#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get()
        .expect("couldn't get db connection from pool");

    // web::block ejecuta operaciones bloqueantes en un thread separado
    match web::block(move || repository::get_all_users(&mut conn)).await {
        Ok(users) => HttpResponse::Ok().json(users.unwrap()),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
```
#### Abstraccion que ayudará a las respuestas y brevedad delc odigo.
```rust
pub fn susses_json<T: Serialize>(body: T) -> HttpResponse{
    return HttpResponse::Ok().json(body)
}
```

Elementos clave:
- `#[get("/users")]`: Macro que define la ruta y método HTTP
- `web::Data<DbPool>`: Inyección de dependencias del pool
- `web::block`: Ejecuta código bloqueante sin bloquear el event loop
- `impl Responder`: Trait que permite diferentes tipos de respuestas HTTP

### Manejo de Peticiones con Path Parameters
```rust
#[get("/users/{id}")]
pub async fn get_user(
    pool: web::Data<DbPool>, 
    id: web::Path<String>
) -> impl Responder {
    let user_id = id.into_inner(); // Extrae el valor del Path
    // ...
}
```

### Manejo de Peticiones con Body
```rust
#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {
    // new_user.into_inner() convierte de Json a NewUser
    // ...
}
```

## Configuración del Servidor

### Main Application (main.rs)
```rust
#[actix_web::main]
async fn main() -> Result<()> {
    // Inicialización de logging
    env_logger::init();
    dotenv().ok();

    // Configuración desde variables de entorno
    let app_config = AppConfig::init_config();

    // Pool de conexiones compartido
    let pool = establish_connection_pool();

    // Configuración del servidor
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Comparte el pool
            .wrap(actix_web::middleware::Logger::default())
            // Registro de servicios
            .service(get_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    });

    // Inicialización del servidor
    server
        .bind((app_config.server.host, app_config.server.port))?
        .run()
        .await
}
```

Elementos importantes:
1. `#[actix_web::main]`: Macro que configura el runtime asíncrono
2. `App::new()`: Crea una nueva instancia de aplicación
3. `.app_data()`: Comparte datos entre servicios
4. `.wrap()`: Agrega middleware global
5. `.service()`: Registra endpoints

## Mejores Prácticas

### 1. Manejo de Errores
```rust
pub enum ApiError {
    DatabaseError(diesel::result::Error),
    NotFound,
    ValidationError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => HttpResponse::NotFound().finish(),
            ApiError::DatabaseError(_) => HttpResponse::InternalServerError().finish(),
            ApiError::ValidationError(msg) => HttpResponse::BadRequest().body(msg.clone()),
        }
    }
}
```

### 2. Middleware Personalizado
```rust
pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware {
    // Implementación del middleware...
}
```

### 3. Configuración de CORS
```rust
use actix_cors::Cors;

App::new()
    .wrap(
        Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600)
    )
```

### 4. Rate Limiting
```rust
use actix_web_httpauth::middleware::HttpAuthentication;

App::new()
    .wrap(
        RateLimiter::new(
            SimpleMemoryBackend::default(),
            RateLimiterConfig::default()
        )
    )
```

## Consideraciones de Rendimiento

1. **Pool de Conexiones**
    - Ajusta el tamaño del pool según tu carga
    - Monitorea las conexiones activas
    - Implementa timeouts apropiados

2. **Operaciones Async**
    - Usa `web::block` para operaciones de DB
    - Evita bloquear el event loop
    - Considera usar streams para datos grandes

3. **Caché**
    - Implementa caché en memoria para datos frecuentes
    - Usa Redis para caché distribuida
    - Configura TTL apropiados

## Logging y Monitoreo

```rust
use log::{info, error, warn};

// En los servicios
info!("Request received: GET /users");
error!("Database error: {}", e);
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_get_users() {
        let pool = establish_test_connection_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(get_users)
        ).await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

Los tests deberían cubrir:
- Rutas HTTP
- Manejo de errores
- Validación de datos
- Integración con la base de datos

## Deployment

1. **Configuración de Producción**
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

2. **Variables de Entorno**
```env
DATABASE_URL=postgresql://user:password@localhost/dbname
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
RUST_LOG=info
```

3. **Dockerfile**
```dockerfile
FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/my_app /usr/local/bin/
CMD ["my_app"]
```

## Conclusiones y Recursos Adicionales

La combinación de Actix-Web con Diesel proporciona:
- Alto rendimiento
- Type safety
- Manejo asíncrono eficiente
- Escalabilidad

Recursos útiles:
- [Documentación de Actix-Web](https://actix.rs/)
- [Ejemplos de Actix](https://github.com/actix/examples)
- [Guía de Diesel](https://diesel.rs/guides/)