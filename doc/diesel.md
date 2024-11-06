# Guía Completa de Implementación de Diesel con Arquitectura por Features en Rust

## Índice
1. [Introducción](#introducción)
2. [Configuración Inicial](#configuración-inicial)
3. [Estructura del Proyecto](#estructura-del-proyecto)
4. [Migraciones](#migraciones)
5. [Implementación por Features](#implementación-por-features)
6. [Testing](#testing)
7. [Mejores Prácticas](#mejores-prácticas)

## Introducción

Diesel es un ORM (Object-Relational Mapping) y Query Builder para Rust que proporciona:
- Type-safety en tiempo de compilación
- Abstracción segura sobre SQL
- Excelente integración con PostgreSQL, MySQL y SQLite

Esta guía se centra en PostgreSQL y utiliza una arquitectura por features, que organiza el código por funcionalidad en lugar de por tipo de componente.

## Configuración Inicial

### 1. Prerequisitos
Asegúrate de tener instalado:
- Rust y Cargo
- PostgreSQL
- Diesel CLI

### 2. Instalación de Diesel CLI
```bash
# Instalamos diesel_cli con soporte solo para PostgreSQL
cargo install diesel_cli --no-default-features --features postgres
```

### 3. Configuración del Proyecto
```bash

# Agregar dependencias en Cargo.toml
[dependencies]
diesel = { version = "2.1.0", features = ["postgres", "chrono", "uuid"] }
dotenv = "0.15.0"
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["serde", "v4"] }
```

### 4. Configuración de la Base de Datos
Crea un archivo `.env` en la raíz del proyecto:
```
DATABASE_URL=postgres://username:password@localhost/dbname
```

## Estructura del Proyecto

```
src/
├── main.rs                   # Punto de entrada de la aplicación
├── db.rs                       # Configuración de la base de datos
├── schema.rs              # Generado por Diesel
└── user/                       # Feature de usuario
      ├── mod.rs              # Exportaciones del módulo
      ├── models.rs         # Definiciones de estructuras
      ├── repository.rs    # Acceso a datos
      └── service.rs         # Lógica de negocio   
```

## Migraciones

### 1. Inicialización
```bash
# Configurar Diesel en el proyecto
diesel setup
```

### 2. Crear Nueva Migración
```bash
# Generar archivos de migración
diesel migration generate create_users
```

### 3. Definir la Migración
En `migrations/[timestamp]_create_users/up.sql`:
```sql
CREATE TABLE users (
    id VARCHAR PRIMARY KEY DEFAULT gen_random_uuid()::text,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Trigger para actualizar updated_at (automáticamente generado)
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

En `migrations/[timestamp]_create_users/down.sql`:
```sql
DROP TABLE users;
```

### 4. Ejecutar Migración
```bash
diesel migration run
```

## Implementación por Features

### 1. Configuración de Base de Datos (src/db.rs)
```rust
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar configurada en el archivo .env");
    PgConnection::establish(&database_url)
        .expect("Error al conectar con la base de datos")
}
```

### 2. Feature User

#### src/features/user/mod.rs
```rust
//! Módulo de usuario que maneja toda la funcionalidad relacionada con usuarios
pub mod models;
pub mod repository;
pub mod service;

// Re-exportaciones para una API más limpia
pub use models::{User, NewUser, UpdateUser};
pub use repository::UserRepository;
pub use service::UserService;
```

#### src/features/user/models.rs
```rust
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(AsChangeset, Deserialize, Validate)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    #[validate(length(min = 2, max = 100))]
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}
```

#### src/features/user/repository.rs
```rust
use diesel::prelude::*;
use diesel::PgConnection;
use crate::schema::users;
use super::models::{User, NewUser, UpdateUser};

pub struct UserRepository {
    conn: PgConnection,
}

impl UserRepository {
    pub fn new(conn: PgConnection) -> Self {
        UserRepository { conn }
    }

    pub fn create(&mut self, new_user: &NewUser) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::insert_into(users)
            .values(new_user)
            .get_result(&mut self.conn)
    }

    pub fn find_by_id(&mut self, user_id: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users.find(user_id).first(&mut self.conn)
    }

    pub fn find_by_email(&mut self, user_email: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        users.filter(email.eq(user_email))
            .first(&mut self.conn)
    }

    pub fn update(&mut self, user_id: &str, user_data: &UpdateUser) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        diesel::update(users.find(user_id))
            .set(user_data)
            .get_result(&mut self.conn)
    }

    pub fn delete(&mut self, user_id: &str) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.find(user_id))
            .execute(&mut self.conn)
    }

    pub fn list(&mut self, limit: i64, offset: i64) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        users
            .order(created_at.desc())
            .limit(limit)
            .offset(offset)
            .load(&mut self.conn)
    }
}
```

#### src/features/user/service.rs
```rust
use super::models::{User, NewUser, UpdateUser};
use super::repository::UserRepository;
use diesel::result::QueryResult;
use validator::Validate;

#[derive(Debug)]
pub enum UserError {
    ValidationError(String),
    DatabaseError(diesel::result::Error),
    UserNotFound,
    EmailTaken,
}

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService { repository }
    }

    pub fn create_user(&mut self, new_user: NewUser) -> Result<User, UserError> {
        // Validar datos de entrada
        new_user.validate()
            .map_err(|e| UserError::ValidationError(e.to_string()))?;

        // Verificar si el email ya existe
        if self.repository.find_by_email(&new_user.email).is_ok() {
            return Err(UserError::EmailTaken);
        }

        // Crear usuario
        self.repository.create(&new_user)
            .map_err(UserError::DatabaseError)
    }

    pub fn get_user(&mut self, id: &str) -> Result<User, UserError> {
        self.repository.find_by_id(id)
            .map_err(|_| UserError::UserNotFound)
    }

    pub fn update_user(&mut self, id: &str, user_data: UpdateUser) -> Result<User, UserError> {
        // Validar datos de entrada
        user_data.validate()
            .map_err(|e| UserError::ValidationError(e.to_string()))?;

        // Verificar si el usuario existe
        self.get_user(id)?;

        // Si se está actualizando el email, verificar que no esté en uso
        if let Some(ref new_email) = user_data.email {
            if let Ok(existing_user) = self.repository.find_by_email(new_email) {
                if existing_user.id != id {
                    return Err(UserError::EmailTaken);
                }
            }
        }

        // Actualizar usuario
        self.repository.update(id, &user_data)
            .map_err(UserError::DatabaseError)
    }

    pub fn delete_user(&mut self, id: &str) -> Result<(), UserError> {
        self.repository.delete(id)
            .map_err(|_| UserError::UserNotFound)?;
        Ok(())
    }

    pub fn list_users(&mut self, page: i64, per_page: i64) -> Result<Vec<User>, UserError> {
        let offset = (page - 1) * per_page;
        self.repository.list(per_page, offset)
            .map_err(UserError::DatabaseError)
    }
}
```

### 3. Uso en main.rs
```rust
use my_project::database::establish_connection;
use my_project::features::user::{UserService, UserRepository, NewUser};

fn main() {
    // Establecer conexión
    let conn = establish_connection();
    
    // Inicializar repositorio y servicio
    let repository = UserRepository::new(conn);
    let mut service = UserService::new(repository);

    // Crear nuevo usuario
    let new_user = NewUser {
        name: String::from("John Doe"),
        email: String::from("john@example.com"),
    };

    match service.create_user(new_user) {
        Ok(user) => println!("Usuario creado: {:?}", user),
        Err(e) => eprintln!("Error al crear usuario: {:?}", e),
    }
}
```

## Testing

### src/features/user/repository_test.rs
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::establish_connection;

    fn setup() -> UserRepository {
        let conn = establish_connection();
        UserRepository::new(conn)
    }

    #[test]
    fn test_create_user() {
        let mut repo = setup();
        let new_user = NewUser {
            name: String::from("Test User"),
            email: String::from("test@example.com"),
        };

        let result = repo.create(&new_user);
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }

    // Más tests...
}
```

## Mejores Prácticas

1. **Manejo de Errores**
    - Crear tipos de error específicos para cada feature
    - Implementar `From` para conversión de errores
    - Usar Result en lugar de panic!

2. **Validación**
    - Validar datos de entrada en la capa de servicio
    - Usar el trait Validate para validaciones declarativas
    - Mantener reglas de negocio centralizadas

3. **Testing**
    - Escribir tests para cada capa (models, repository, service)
    - Usar fixtures para datos de prueba
    - Implementar tests de integración

4. **Organización del Código**
    - Mantener archivos pequeños y enfocados
    - Documentar funciones públicas
    - Usar re-exportaciones para una API limpia

5. **Rendimiento**
    - Implementar paginación para listas largas
    - Usar índices en la base de datos
    - Considerar el uso de conexiones pooling

## Notas Adicionales

1. **Transacciones**
```rust
use diesel::result::QueryResult;
use diesel::Connection;

pub fn create_related_data(conn: &mut PgConnection) -> QueryResult<()> {
    conn.transaction(|conn| {
        // Operaciones dentro de la transacción
        Ok(())
    })
}
```

2. **Logging**
```rust
use log::{info, error};

info!("Usuario creado: {}", user.id);
error!("Error al crear usuario: {}", e);
```

3. **Conexiones Pooling**
```rust
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
```