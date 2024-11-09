use actix_web::{HttpResponse, Responder};
use actix_web::web::Data;
use uuid::Uuid;
use crate::model::AppState;
use super::models::{NewUser, Users};

pub struct UserRepository;

impl UserRepository {
    pub  async fn create_user(conn: Data<AppState>, new_user: NewUser) -> impl Responder {
        
        match sqlx::query(
        "INSERT INTO users (id, first_name, last_name, email, password_hash, position, department, phone, status, email_notification, push_notification, auto_sync, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)")
            .bind(new_user.id)
            .bind(new_user.first_name)
            .bind(new_user.last_name)
            .bind(new_user.email)
            .bind(new_user.password_hash)
            .bind(new_user.position)
            .bind(new_user.department)
            .bind(new_user.phone)
            .bind(new_user.status)
            .bind(new_user.email_notification)
            .bind(new_user.push_notification)
            .bind(new_user.auto_sync)
            .bind(new_user.created_at)
            .bind(new_user.updated_at)
            .execute(&conn.db)
            .await
        {
            Ok(_) => HttpResponse::Created().json("User created successfully"), 
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
}
