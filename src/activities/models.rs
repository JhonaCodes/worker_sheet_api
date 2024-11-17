use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Activities {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub risk_level: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub user_id: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_synchronized: bool,
    pub hashtag: Option<String>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PhotoActivity{
    pub id: String,
    pub activity_id: String,
    pub url: String,
}




#[derive(Serialize, Deserialize, FromRow)]
pub struct ActivityFilter {
    pub status: Option<String>,
    pub risk_level: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub user_id: Option<String>,
    pub is_synchronized: Option<bool>,
    pub hashtag: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewPhoto {
    pub activity_id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UpdateActivityStatus {
    pub status: String,
    pub updated_at: Option<NaiveDateTime>,
}
