use std::error::Error;
use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};
use argonautica::Hasher;
use chrono::{NaiveDateTime, Utc};

use uuid::Uuid;
use crate::activities::models::Activities;
use crate::auth::env::hash_secret;
use crate::helper::response::send_email;
use crate::model::AppState;
use super::models::{UserModel, UpdateUser, UpdateUserNotifications, UpdateUserPassword, UpdateUserStatus, UserFilters, Users};

pub struct UserRepository;

impl UserRepository {
    pub  async fn create_user(conn: Data<AppState>, body: Json<UserModel>) -> impl Responder {

        let new_user: UserModel = body.into_inner();

        let hash = Hasher::default()
            .with_password(new_user.password_hash)
            .with_secret_key(hash_secret())
            .hash()
            .unwrap();


        match sqlx::query(
        "INSERT INTO users (id, first_name, last_name, email, password_hash, position, department, phone, status, email_notification, push_notification, auto_sync, created_at, hash_sync) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)")
            .bind(new_user.id)
            .bind(new_user.first_name)
            .bind(new_user.last_name)
            .bind(new_user.email.clone())
            .bind(hash)
            .bind(new_user.position)
            .bind(new_user.department)
            .bind(new_user.phone)
            .bind(new_user.status)
            .bind(new_user.email_notification)
            .bind(new_user.push_notification)
            .bind(new_user.auto_sync)
            .bind(Utc::now())
            .bind(Utc::now().timestamp_millis().to_string())
            .execute(&conn.db)
            .await
        {
            Ok(_) =>{

                match send_email(&new_user.email) {
                    Ok(_) => {
                        HttpResponse::Created().json("User created successfully")
                    }
                    Err(e) => {
                        HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
                    }
                }
            
                
            }, 
            Err(e) => {
                log::error!("Error creating user: {:?}", e); 
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }
    
    pub async fn get_by_id( conn: Data<AppState>, user_id: Uuid) -> impl Responder {
        match sqlx::query_as::<_,Users>("SELECT * FROM users WHERE id = $1" )
            .bind(user_id)
            .fetch_one(&conn.db).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(e) => {
                log::error!("Error getting user: {}", e);
                HttpResponse::InternalServerError().json(format!("Error: {}", e))
            }
        }
    }


    // Listado de usuarios con filtros
    pub async fn list_users(conn: Data<AppState>, filters: UserFilters) -> impl Responder {
        let query = sqlx::query_as::<_, Users>(
            "SELECT * FROM users
            WHERE ($1::text IS NULL OR department = $1)
            AND ($2::text IS NULL OR position = $2)
            AND ($3::text IS NULL OR status = $3)
            AND ($4::timestamp IS NULL OR created_at >= $4)
            AND ($5::timestamp IS NULL OR created_at <= $5)
            ORDER BY created_at DESC"
        )
            .bind(filters.department)
            .bind(filters.position)
            .bind(filters.status)
            .bind(filters.created_from)
            .bind(filters.created_to);

        match query.fetch_all(&conn.db).await {
            Ok(users) => HttpResponse::Ok().json(users),
            Err(e) => {
                log::error!("Error listing users: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    // Actualizar usuario completo
    pub async fn update_user(
        conn: Data<AppState>,
        id: Uuid,
        user: UpdateUser
    ) -> impl Responder {
        match sqlx::query(
            "UPDATE users SET
            first_name = $1,
            last_name = $2,
            email = $3,
            position = $4,
            department = $5,
            phone = $6,
            status = $7,
            email_notification = $8,
            push_notification = $9,
            hash_sync = $10
            WHERE id = $11"
        )
            .bind(user.first_name)
            .bind(user.last_name)
            .bind(user.email)
            .bind(user.position)
            .bind(user.department)
            .bind(user.phone)
            .bind(user.status)
            .bind(user.email_notification)
            .bind(user.push_notification)
            .bind(user.hash_sync)
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Ok().json("User updated successfully"),
            Err(e) => {
                log::error!("Error updating user: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    // Actualizar estado del usuario
    pub async fn update_user_status(
        conn: Data<AppState>,
        id: Uuid,
        status_update: UpdateUserStatus
    ) -> impl Responder {
        match sqlx::query(
            "UPDATE users SET status = $1, updated_at = $2 WHERE id = $3"
        )
            .bind(status_update.status)
            .bind(status_update.updated_at)
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Ok().json("Status updated successfully"),
            Err(e) => {
                log::error!("Error updating status: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    // Actualizar notificaciones
    pub async fn update_user_notifications(
        conn: Data<AppState>,
        id: Uuid,
        notifications: UpdateUserNotifications
    ) -> impl Responder {
        match sqlx::query(
            "UPDATE users SET
            email_notification = $1,
            push_notification = $2,
            auto_sync = $3,
            updated_at = $4
            WHERE id = $5"
        )
            .bind(notifications.email_notification)
            .bind(notifications.push_notification)
            .bind(notifications.auto_sync)
            .bind(notifications.hash_sync)
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Ok().json("Notifications updated successfully"),
            Err(e) => {
                log::error!("Error updating notifications: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    // Actualizar contraseña
    pub async fn update_user_password(
        conn: Data<AppState>,
        id: Uuid,
        password_update: UpdateUserPassword
    ) -> impl Responder {
        // En una implementación real, aquí verificarías la contraseña actual
        match sqlx::query(
            "UPDATE users SET password_hash = $1, updated_at = $2 WHERE id = $3"
        )
            .bind(password_update.new_password)
            .bind(password_update.hash_sync)
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Ok().json("Password updated successfully"),
            Err(e) => {
                log::error!("Error updating password: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    // Obtener actividades del usuario
    pub async fn get_user_activities(
        conn: Data<AppState>,
        user_id: Uuid,
        from_date: Option<NaiveDateTime>,
        to_date: Option<NaiveDateTime>
    ) -> impl Responder {
        let query = sqlx::query_as::<_, Activities>(
            "SELECT * FROM activities
            WHERE user_id = $1
            AND ($2::timestamp IS NULL OR created_at >= $2)
            AND ($3::timestamp IS NULL OR created_at <= $3)
            AND is_deleted = false
            ORDER BY created_at DESC"
        )
            .bind(user_id.to_string())
            .bind(from_date)
            .bind(to_date);

        match query.fetch_all(&conn.db).await {
            Ok(activities) => HttpResponse::Ok().json(activities),
            Err(e) => {
                log::error!("Error getting user activities: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }
    
}
