use actix_web::{Responder};
use actix_web::web::{Data, Json};
use argonautica::Hasher;
use chrono::{NaiveDateTime, Utc};

use uuid::Uuid;
use crate::activities::models::Activities;
use crate::auth::env::hash_secret;

use crate::helper::email_service_helper::{send_email, susses_json, un_success_json};
use crate::helper::validation_helper::ValidateHelper;
use crate::model::AppState;
use super::models::{UserModel, UpdateUser, UpdateUserNotifications, UpdateUserPassword, UpdateUserStatus, UserFilters, Users};

pub struct UserRepository;

impl UserRepository {
    pub  async fn create_user(conn: Data<AppState>, body: Json<UserModel>) -> impl Responder {
        let new_user: UserModel = body.into_inner();

        if ValidateHelper::is_valid_email(&new_user.email) {

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
                Ok(_) => {
                    match send_email(&new_user.email) {
                        Ok(_) => {
                            susses_json("User created successfully")
                        }
                        Err(_) => un_success_json(
                            "Email delivery error",
                            Some("No se pudo enviar el correo electrónico. Por favor, verifica tu dirección de email o inténtalo más tarde")
                        )
                    }
                },
                Err(_) => un_success_json(
                    "User registration error",
                    Some("No se pudo completar el registro del usuario. Por favor, inténtalo nuevamente más tarde")
                )
            }

        }else {
            un_success_json(
                "Invalid User DataError",
                Some("Los datos proporcionados del usuario son incorrectos o están incompletos")
            )
        }


}

pub async fn get_by_id(conn: Data<AppState>, user_id: Uuid) -> impl Responder {
        match sqlx::query_as::<_,Users>("SELECT * FROM users WHERE id = $1" )
            .bind(user_id)
            .fetch_one(&conn.db).await {
            Ok(user) => susses_json(user),
            Err(_) => un_success_json(
                "Error on calling users",
                Some("Error al llamar listado de usuarios")
            )
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
            Ok(users) => susses_json(users),
            Err(_) => un_success_json(
                "Error al listar usuarios",
                Some("No se pudo obtener el listado de usuarios del sistema")
            )
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
            Ok(_) => susses_json("User updated successfully"),
            Err(_) => un_success_json(
                "Error al actualizar usuario",
                Some("No se pudo actualizar la información del usuario en el sistema")
            )
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
            Ok(_) => susses_json("Status updated successfully"),
            Err(_) =>un_success_json(
                "Error al actualizar estado",
                Some("No se pudo actualizar el estado en el sistema")
            )
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
            Ok(_) => susses_json("Notifications updated successfully"),
            Err(_) => un_success_json(
                "Error al actualizar notificaciones",
                Some("No se pudo actualizar las notificaciones en el sistema")
            )
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
            Ok(_) => susses_json("Password updated successfully"),
            Err(_) => un_success_json(
                "Error al actualizar contraseña",
                Some("No se pudo actualizar la contraseña del usuario en el sistema")
            )
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
            Ok(activities) => susses_json(activities),
            Err(_) => un_success_json(
                "Error al obtener actividades",
                Some("No se pudo recuperar las actividades del usuario del sistema")
            )
        }
    }


    pub async fn remove_user(conn: Data<AppState>, id:Uuid, mail:String)-> impl Responder {
        match sqlx::query(r#"UPDATE users SET status = $1 WHERE id = $2 AND email = $3"#)
            .bind("deleted")
            .bind(id.clone())
            .bind(mail.clone())
            .execute(&conn.db)
            .await {
            Ok(_) => susses_json("Usuario eliminado exitosamente"),
            Err(_) => un_success_json(
                "Error al eliminar usuario",
                Some("No se pudo eliminar el usuario del sistema")
            )

        }
    }
    
}
