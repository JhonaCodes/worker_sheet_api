use actix_web::web::{Path};
use actix_web::{get, post, Responder};
use actix_web::web::{Data, Json};
use uuid::Uuid;
use crate::model::AppState;
use crate::participants::models::ParticipantsModel;
use crate::participants::repository::ParticipantsRepository;

#[post("/participant")]
pub async fn create_participant(conn: Data<AppState>, body: Json<ParticipantsModel>)->impl Responder{
    return ParticipantsRepository::assign_participant(conn, body).await;
}
#[get("/participant/{id}")]
pub async fn get_participants(conn: Data<AppState>, id_activity: Path<Uuid>) -> impl Responder {
    return ParticipantsRepository::get_participants(conn, id_activity.into_inner()).await;
}

#[post("/activities/{user_id}")]
pub async fn get_activities_by_participant_id(conn: Data<AppState>, user_id: Path<Uuid>) -> impl  Responder{
    return ParticipantsRepository::get_activities_by_participant_id(conn, user_id.into_inner()).await;
}