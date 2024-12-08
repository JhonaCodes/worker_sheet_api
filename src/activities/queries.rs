use std::fmt::format;
use actix_web::Responder;
use actix_web::web::Data;
use uuid::Uuid;
use crate::activities::models::ActivitiesWithPhoto;
use crate::helper::email_service_helper::{susses_json, un_success_json};
use crate::model::AppState;

pub struct ActivityQueries;


impl ActivityQueries {
    const SELECT_ACTIVITIES_WITH_PHOTOS: &'static str = r#"
       SELECT a.*, (
            SELECT array_agg(ap.url)
            FROM activity_photos ap
            WHERE ap.activity_id = a.id
        ) as photos
        FROM activities a"#;


    pub async fn local_activity_with_photo(conn: Data<AppState>,  id: Uuid, conditional_query: &str)-> impl Responder {

        let query = format!(r#"{} {}"#, Self::SELECT_ACTIVITIES_WITH_PHOTOS, conditional_query);

        println!("Final query {}", query);
        let activities_result = sqlx::query_as::<_, ActivitiesWithPhoto>(&*query)
            .bind(id)
            .fetch_all(&conn.db)
            .await;
        match activities_result {
            Ok(activities) => {
                // Convertimos los resultados al formato deseado
                let response: Vec<ActivitiesWithPhoto> = activities
                    .into_iter()
                    .map(|activity| ActivitiesWithPhoto {
                        id: activity.id,
                        title: activity.title,
                        description: activity.description,
                        status: activity.status,
                        risk_level: activity.risk_level,
                        location_lat: activity.location_lat,
                        location_lng: activity.location_lng,
                        user_id: activity.user_id,
                        start_date: activity.start_date,
                        end_date: activity.end_date,
                        created_at: activity.created_at,
                        updated_at: activity.updated_at,
                        hash_sync: activity.hash_sync,
                        is_deleted: activity.is_deleted,
                        photos: activity.photos,
                    }).collect();

                susses_json(response)
            }
            Err(err) => {
                println!("Error: {}", err.to_string());
                un_success_json(
                    "No hay actividades",
                    Some("No se encontraron actividades relacionadas")
                )
            }
        }
    }
}