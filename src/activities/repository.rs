use actix_web::{HttpResponse, Responder, web::Data};
use uuid::Uuid;
use crate::model::AppState;
use super::models::{Activities, ActivityFilter, NewPhoto, PhotoActivity, UpdateActivityStatus};

pub struct ActivityRepository;

impl ActivityRepository {
    // CRUD Básico
    pub async fn create_activity(conn: Data<AppState>, new_activity: Activities) -> impl Responder {
        match sqlx::query(
            "INSERT INTO activities
            (id, title, description, status, risk_level, location_lat, location_lng,
             user_id, start_date, end_date, created_at, updated_at, is_synchronized, hashtag, is_deleted)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"
        )
            .bind(new_activity.id)
            .bind(new_activity.title)
            .bind(new_activity.description)
            .bind(new_activity.status)
            .bind(new_activity.risk_level)
            .bind(new_activity.location_lat)
            .bind(new_activity.location_lng)
            .bind(new_activity.user_id)
            .bind(new_activity.start_date)
            .bind(new_activity.end_date)
            .bind(new_activity.created_at)
            .bind(new_activity.updated_at)
            .bind(new_activity.is_synchronized)
            .bind(new_activity.hashtag)
            .bind(new_activity.is_deleted)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Created().json("Activity created successfully"),
            Err(e) => {
                log::error!("Error creating activity: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    pub async fn get_activity_by_id(conn: Data<AppState>, activity_id: Uuid) -> impl Responder {
        match sqlx::query_as::<_, Activities>(
            "SELECT * FROM activities WHERE id = $1 AND is_deleted = false"
        )
            .bind(activity_id)
            .fetch_one(&conn.db)
            .await {
            Ok(activity) => HttpResponse::Ok().json(activity),
            Err(e) => {
                log::error!("Error getting activity: {:?}", e);
                HttpResponse::NotFound().json(format!("Activity not found: {:?}", e))
            }
        }
    }

    pub async fn list_activities(conn: Data<AppState>, filter: ActivityFilter) -> impl Responder {
        let query = sqlx::query_as::<_, Activities>(
            "SELECT * FROM activities
            WHERE ($1::text IS NULL OR status = $1)
            AND ($2::text IS NULL OR risk_level = $2)
            AND ($3::timestamp IS NULL OR start_date >= $3)
            AND ($4::timestamp IS NULL OR end_date <= $4)
            AND ($5::text IS NULL OR user_id = $5)
            AND ($6::boolean IS NULL OR is_synchronized = $6)
            AND ($7::text IS NULL OR hashtag = $7)
            AND is_deleted = false
            ORDER BY created_at DESC"
        )
            .bind(filter.status)
            .bind(filter.risk_level)
            .bind(filter.start_date)
            .bind(filter.end_date)
            .bind(filter.user_id)
            .bind(filter.is_synchronized)
            .bind(filter.hashtag);

        match query.fetch_all(&conn.db).await {
            Ok(activities) => HttpResponse::Ok().json(activities),
            Err(e) => {
                log::error!("Error listing activities: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    pub async fn update_activity(
        conn: Data<AppState>,
        id: Uuid,
        activity: Activities
    ) -> impl Responder {
        match sqlx::query(
            "UPDATE activities SET
            title = $1, description = $2, status = $3, risk_level = $4,
            location_lat = $5, location_lng = $6, start_date = $7, end_date = $8,
            updated_at = $9, is_synchronized = $10, hashtag = $11
            WHERE id = $12 AND is_deleted = false"
        )
            .bind(activity.title)
            .bind(activity.description)
            .bind(activity.status)
            .bind(activity.risk_level)
            .bind(activity.location_lat)
            .bind(activity.location_lng)
            .bind(activity.start_date)
            .bind(activity.end_date)
            .bind(activity.updated_at)
            .bind(activity.is_synchronized)
            .bind(activity.hashtag)
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Ok().json("Activity updated successfully"),
            Err(e) => {
                log::error!("Error updating activity: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    pub async fn update_activity_status(
        conn: Data<AppState>,
        id: Uuid,
        status_update: UpdateActivityStatus
    ) -> impl Responder {
        match sqlx::query(
            "UPDATE activities SET status = $1, updated_at = $2 WHERE id = $3 AND is_deleted = false"
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

    pub async fn delete_activity(conn: Data<AppState>, id: Uuid) -> impl Responder {
        match sqlx::query(
            "UPDATE activities SET is_deleted = true WHERE id = $1"
        )
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Ok().json("Activity deleted successfully"),
            Err(e) => {
                log::error!("Error deleting activity: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    // Fotos
    pub async fn add_photo(conn: Data<AppState>, photo: NewPhoto) -> impl Responder {
        match sqlx::query(
            "INSERT INTO activity_photos (activity_id, url) VALUES ($1, $2)"
        )
            .bind(photo.activity_id)
            .bind(photo.url)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Created().json("Photo added successfully"),
            Err(e) => {
                log::error!("Error adding photo: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    pub async fn get_activity_photos(conn: Data<AppState>, activity_id: String) -> impl Responder {
        match sqlx::query_as::<_, PhotoActivity>(
            "SELECT * FROM activity_photos WHERE activity_id = $1"
        )
            .bind(activity_id)
            .fetch_all(&conn.db)
            .await {
            Ok(photos) => HttpResponse::Ok().json(photos),
            Err(e) => {
                log::error!("Error getting photos: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }

    pub async fn delete_photo(
        conn: Data<AppState>,
        activity_id: String,
        photo_id: i32
    ) -> impl Responder {
        match sqlx::query(
            "DELETE FROM activity_photos WHERE activity_id = $1 AND id = $2"
        )
            .bind(activity_id)
            .bind(photo_id)
            .execute(&conn.db)
            .await {
            Ok(_) => HttpResponse::Ok().json("Photo deleted successfully"),
            Err(e) => {
                log::error!("Error deleting photo: {:?}", e);
                HttpResponse::InternalServerError().json(format!("Error: {:?}", e))
            }
        }
    }
    
    
}