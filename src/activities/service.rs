use actix_web::{delete, get, patch, post, put, web::{self, Json, Path, Query}, Responder};
use uuid::Uuid;
use crate::model::AppState;
use super::{models::{Activities, ActivityFilter, NewPhoto, UpdateActivityStatus}, repository::ActivityRepository};

#[post("/activities")]
pub async fn create_activity(
    conn: web::Data<AppState>,
    new_activity: Json<Activities>
) -> impl Responder {
    ActivityRepository::create_activity(conn, new_activity.into_inner()).await
}

#[get("/activities/{id}")]
pub async fn get_activity(
    conn: web::Data<AppState>,
    id: Path<Uuid>
) -> impl Responder {
    ActivityRepository::get_activity_by_id(conn, id.into_inner()).await
}

#[get("/activities")]
pub async fn list_activities(
    conn: web::Data<AppState>,
    filter: Query<ActivityFilter>
) -> impl Responder {
    ActivityRepository::list_activities(conn, filter.into_inner()).await
}

#[put("/activities/{id}")]
pub async fn update_activity(
    conn: web::Data<AppState>,
    id: Path<Uuid>,
    activity: Json<Activities>
) -> impl Responder {
    ActivityRepository::update_activity(conn, id.into_inner(), activity.into_inner()).await
}

#[patch("/activities/{id}/status")]
pub async fn update_activity_status(
    conn: web::Data<AppState>,
    id: Path<Uuid>,
    status: Json<UpdateActivityStatus>
) -> impl Responder {
    ActivityRepository::update_activity_status(conn, id.into_inner(), status.into_inner()).await
}

#[delete("/activities/{id}")]
pub async fn delete_activity(
    conn: web::Data<AppState>,
    id: Path<Uuid>
) -> impl Responder {
    ActivityRepository::delete_activity(conn, id.into_inner()).await
}

#[post("/activities/{id}/photos")]
pub async fn add_photo(
    conn: web::Data<AppState>,
    photo: Json<NewPhoto>
) -> impl Responder {
    ActivityRepository::add_photo(conn, photo.into_inner()).await
}

#[get("/activities/{id}/photos")]
pub async fn get_photos(
    conn: web::Data<AppState>,
    id: Path<String>
) -> impl Responder {
    ActivityRepository::get_activity_photos(conn, id.into_inner()).await
}

#[delete("/activities/{activity_id}/photos/{photo_id}")]
pub async fn delete_photo(
    conn: web::Data<AppState>,
    path: Path<(String, i32)>
) -> impl Responder {
    let (activity_id, photo_id) = path.into_inner();
    ActivityRepository::delete_photo(conn, activity_id, photo_id).await
}
