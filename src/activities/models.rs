use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Activities {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub risk_level: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub hash_sync: Option<String>,
    pub is_deleted: Option<bool>
}

#[derive(Serialize, Deserialize, FromRow, Decode)]
pub struct ActivitiesWithPhoto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub risk_level: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub hash_sync: Option<String>,
    pub is_deleted: Option<bool>,
    pub photos: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PhotoActivity{
    pub id: i32,
    pub activity_id: Uuid,
    pub url: String,
}




#[derive(Serialize, Deserialize, FromRow)]
pub struct ActivityFilter {
    pub status: Option<String>,
    pub risk_level: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub user_id: Option<Uuid>,
    pub hash_sync: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct NewPhoto {
    pub activity_id: Uuid,
    pub url: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UpdateActivityStatus {
    pub status: String,
    pub hash_sync: Option<String>,
}
