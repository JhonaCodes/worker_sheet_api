use actix_multipart::Multipart;
use actix_web::{delete, get, patch, post, put, web::{Json, Path, Query}, Responder};
use actix_web::web::Data;
use uuid::Uuid;
use crate::model::AppState;
use super::{models::{Activities, ActivityFilter, UpdateActivityStatus}, repository::ActivityRepository};

#[post("/activities")]
pub async fn create_activity(
    conn: Data<AppState>,
    new_activity: Json<Activities>
) -> impl Responder {
    ActivityRepository::create_activity(conn, new_activity.into_inner()).await
}

#[get("/activities/{user_id}/limit/{limit}")]
pub async fn get_activity(conn: Data<AppState>, user_id: Path<Uuid>, limit: Path<i32>) -> impl Responder {
    ActivityRepository::get_activity_by_user_id(conn, user_id.into_inner(), limit.into_inner()).await
}

#[get("/activities/participant/{user_id}")]
pub async fn get_activity_list_by_user_id(conn:Data<AppState>, user_id:Path<Uuid>) -> impl Responder {
    ActivityRepository::get_activity_by_participant(conn, user_id.into_inner()).await
}

#[get("/activities")]
pub async fn list_activities(
    conn: Data<AppState>,
    filter: Query<ActivityFilter>
) -> impl Responder {
    ActivityRepository::list_activities(conn).await
}

#[put("/activities/{id}")]
pub async fn update_activity(
    conn: Data<AppState>,
    id: Path<Uuid>,
    activity: Json<Activities>
) -> impl Responder {
    ActivityRepository::update_activity(conn, id.into_inner(), activity.into_inner()).await
}

#[patch("/activities/{id}/status")]
pub async fn update_activity_status(
    conn: Data<AppState>,
    id: Path<Uuid>,
    status: Json<UpdateActivityStatus>
) -> impl Responder {
    ActivityRepository::update_activity_status(conn, id.into_inner(), status.into_inner()).await
}

#[delete("/activities/{id}")]
pub async fn delete_activity(
    conn: Data<AppState>,
    id: Path<Uuid>
) -> impl Responder {
    ActivityRepository::delete_activity(conn, id.into_inner()).await
}

#[post("/activities/{id}/photos")]
pub async fn add_photo( conn: Data<AppState>,  activity_id: Path<Uuid>, payload: Multipart, ) -> impl Responder {
    println!("Add photo {}", activity_id);
    ActivityRepository::add_photo(conn, activity_id.into_inner(), payload).await
}

#[get("/activities/{id}/photos")]
pub async fn get_photos(
    conn: Data<AppState>,
    id: Path<String>
) -> impl Responder {
    ActivityRepository::get_activity_photos(conn, id.into_inner()).await
}

#[delete("/activities/{activity_id}/photos/{photo_id}")]
pub async fn delete_photo(
    conn: Data<AppState>,
    path: Path<(String, i32)>
) -> impl Responder {
    let (activity_id, photo_id) = path.into_inner();
    ActivityRepository::delete_photo(conn, activity_id, photo_id).await
}
