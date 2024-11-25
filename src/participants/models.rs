use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Deserialize, FromRow, Serialize)]
pub struct ParticipantsModel{
    id: i32,
    pub  activity_id: Uuid,
    pub user_id: Uuid
}