use std::collections::HashMap;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web::Data};
use actix_web::web::BufMut;
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use sqlx::Postgres;
use uuid::Uuid;
use crate::helper::email_service_helper::{susses_json, un_success_json};
use crate::model::AppState;
use super::models::{Activities, ActivityFilter, NewPhoto, PhotoActivity, UpdateActivityStatus};

pub struct ActivityRepository;

impl ActivityRepository {
    // CRUD Básico
    pub async fn create_activity(conn: Data<AppState>, new_activity: Activities) -> impl Responder {
        match sqlx::query_as::<_, Activities>(
            "INSERT INTO activities
            (id, title, description, status, risk_level, location_lat, location_lng,
             user_id, start_date, end_date, created_at, updated_at, hash_sync, is_deleted)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14) RETURNING *"
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
            .bind(new_activity.hash_sync)
            .bind(new_activity.is_deleted)
            .fetch_one(&conn.db)
            .await {
            Ok(activity) => susses_json(activity),
            Err(_) => un_success_json(
                "No se pudo registrar la cuenta",
                Some("No se pudo crear el usuario. Por favor, verifica los datos ingresados e inténtalo nuevamente")
            )
        }
    }

    // pub async fn get_activity_by_user_id(conn: Data<AppState>, user_id: Uuid) -> impl Responder {
    //     match sqlx::query_as::<_, Activities>(
    //         "SELECT * FROM activities WHERE user_id = $1 AND is_deleted = false"
    //     )
    //         .bind(user_id.to_string())
    //         .fetch_all(&conn.db)
    //         .await {
    //         Ok(activity) => susses_json(activity),
    //         Err(_) => un_success_json(
    //             "No hay actividades",
    //             Some("No se encontraron actividades relacionadas")
    //         )
    //     }
    // }


    pub async fn get_activity_by_user_id(conn: Data<AppState>, user_id: Uuid, limit: usize) -> impl  Responder {
        let limit_page = if limit > 10 { limit } else { 10 };

        // Primero obtenemos las actividades
        let activities = match sqlx::query_as::<_, Activities>(r#"SELECT * FROM activities WHERE user_id = $1 AND is_deleted = false"#)
            .bind(user_id.to_string())
            .limit(limit_page)
            .fetch_all(&conn.db)
            .await {
            Ok(acts) => acts,
            Err(_) => return un_success_json(
                "No hay actividades",
                Some("No se encontraron actividades relacionadas")
            ),
        };

        // Luego obtenemos todas las fotos
        let photos = match sqlx::query_as::<_, PhotoActivity>(r#"
        SELECT ph.*
        FROM activity_photos ph
        INNER JOIN activities ac ON ac.id = ph.activity_id
        WHERE ph.user_id = $1"#)
            .bind(user_id)
            .fetch_all(&conn.db)
            .await {
            Ok(photos) => photos,
            Err(_) => return un_success_json(
                "No hay imágenes",
                Some("No se encontraron imágenes relacionadas a esta actividad")
            ),
        };

        // Convertimos cada actividad a un Value y le agregamos sus fotos
        let activities_with_photos: Vec<serde_json::Value> = activities.into_iter()
            .map(|activity| {
                let mut activity_json = serde_json::to_value(&activity).unwrap_or_default();

                // Encontramos todas las fotos que corresponden a esta actividad
                let activity_photo_urls: Vec<String> = photos.iter()
                    .filter(|photo| photo.activity_id == activity.id)
                    .map(|photo| photo.url.clone())
                    .collect();

                // Agregamos el array de URLs al JSON de la actividad
                if let serde_json::Value::Object(ref mut map) = activity_json {
                    map.insert(
                        "photos".to_string(),
                        serde_json::to_value(activity_photo_urls).unwrap_or_default()
                    );
                }

                activity_json
            })
            .collect();

        // Retornamos el array de actividades
        susses_json(activities_with_photos)
    }
    pub async fn get_activity_by_participant(conn: Data<AppState>, user_id: Uuid) -> impl Responder {
        match sqlx::query_as::<_, Activities>(r#"SELECT act.*
            FROM activities act
            INNER JOIN participants part ON act.id = part.activity_id
            WHERE part.user_id = $1;"#
        ).bind(user_id).fetch_all(&conn.db).await {
            Ok(activity_list)=> susses_json(activity_list),
            Err(_)=> un_success_json("Error al llamar actividades", Some("No se pudo encontrar actividades relacionadas."))
        }
    }

    pub async fn list_activities(conn: Data<AppState>) -> impl Responder {
        let query = sqlx::query_as::<_, Activities>(
            "SELECT * FROM activities"
        );

        match query.fetch_all(&conn.db).await {
            Ok(activities) => susses_json(activities),
            Err(_) => un_success_json(
                "Error en búsqueda de actividades",
                Some("No se encontraron actividades asociadas a este usuario")
            )
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
            updated_at = $9, hash_sync = $10
            WHERE id = $11 AND is_deleted = false"
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
            .bind(activity.hash_sync)
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => susses_json("Activity updated successfully"),
            Err(_) => un_success_json(
                "Error al actualizar actividad",
                Some("No se pudo actualizar la actividad solicitada")
            )
        }
    }

    pub async fn update_activity_status(
        conn: Data<AppState>,
        id: Uuid,
        status_update: UpdateActivityStatus
    ) -> impl Responder {
        match sqlx::query(
            "UPDATE activities SET status = $1, hash_sync = $2 WHERE id = $3 AND is_deleted = false"
        )
            .bind(status_update.status)
            .bind(status_update.hash_sync)
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => susses_json("Status updated successfully"),
            Err(_) => un_success_json(
                "Error al cambiar estado",
                Some("No se pudo actualizar el estado de la actividad")
            )
        }
    }

    pub async fn delete_activity(conn: Data<AppState>, id: Uuid) -> impl Responder {
        match sqlx::query(
            "UPDATE activities SET is_deleted = true WHERE id = $1"
        )
            .bind(id)
            .execute(&conn.db)
            .await {
            Ok(_) => susses_json("Activity deleted successfully"),
            Err(_) => un_success_json(
                "Error al eliminar actividad",
                Some("No se pudo eliminar la actividad del sistema")
            )
        }
    }

    pub async fn add_photo(conn: Data<AppState>, activity_id: Uuid, mut payload: Multipart) -> impl Responder {
        if let Err(e) = fs::create_dir_all("/app/uploads") {
            log::error!("Error creating directory: {:?}", e);
            return un_success_json(
                "Error creando directorio",
                Some("No se pudo crear el directorio de uploads")
            );
        }

        while let Ok(Some(mut field)) = payload.try_next().await {
            let file_name = format!("{}.jpg", Uuid::new_v4());
            let file_path = format!("/app/uploads/{}", file_name);
            let url_path = format!("/uploads/{}", file_name);

            let file = match fs::File::create(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    log::error!("Error creating file: {:?}", e);
                    return un_success_json(
                        "Error creando archivo",
                        Some("No se pudo crear el archivo")
                    );
                }
            };

            let mut file = BufWriter::new(file);

            while let Ok(Some(chunk)) = field.try_next().await {
                if let Err(e) = file.write_all(&chunk) {
                    log::error!("Error writing file: {:?}", e);
                    return un_success_json(
                        "Error escribiendo archivo",
                        Some("No se pudo escribir el archivo")
                    );
                }
            }

            return match sqlx::query(
                "INSERT INTO activity_photos (activity_id, url) VALUES ($1, $2)"
            )
                .bind(activity_id)
                .bind(&url_path)
                .execute(&conn.db)
                .await {
                Ok(_) => susses_json(json!({ "message": "Photo added successfully", "url": url_path})),
                Err(_) => un_success_json(
                    "Error al guardar en base de datos",
                    Some("No se pudo guardar el archivo en el sistema debido a un error en la base de datos")
                )
            };
        }

        un_success_json(
            "Error en la solicitud",
            Some("No se proporcionó ningún archivo")
        )
    }

    pub async fn get_activity_photos(conn: Data<AppState>, activity_id: String) -> impl Responder {
        match sqlx::query_as::<_, PhotoActivity>(
            "SELECT * FROM activity_photos WHERE activity_id = $1"
        )
            .bind(activity_id)
            .fetch_all(&conn.db)
            .await {
            Ok(photos) => susses_json(photos),
            Err(_) => un_success_json(
                "Error al obtener fotos",
                Some("No se pudieron recuperar las fotos del sistema")
            )
        }
    }

    pub async fn delete_photo( conn: Data<AppState>,  activity_id: String,  photo_id: i32 ) -> impl Responder {

        match sqlx::query(r#"DELETE FROM activity_photos WHERE activity_id = $1 AND id = $2"#)
            .bind(activity_id)
            .bind(photo_id)
            .execute(&conn.db)
            .await {
            Ok(_) => susses_json("Photo deleted successfully"),
            Err(_) => un_success_json("Error al eliminar foto",  Some("No se pudo eliminar la foto del sistema") )

        }

    }
    
    
}