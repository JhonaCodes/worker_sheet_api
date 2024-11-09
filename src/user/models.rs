use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Users {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub position: String,
    pub department: String,
    pub phone: String,
    pub status: String,
    pub email_notification: Option<bool>,
    pub push_notification: Option<bool>,
    pub auto_sync: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive( Deserialize, Serialize, FromRow)]
pub struct NewUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub position: String,
    pub department: String,
    pub phone: String,
    pub status: String,
    pub email_notification:bool,
    pub push_notification:bool,
    pub auto_sync: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}


#[derive( Deserialize)]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub position: String,
    pub department: String,
    pub phone: String,
    pub status: String,
    pub email_notification:bool,
    pub push_notification:bool,
    pub auto_sync: bool,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct PhotoActivity{
    pub id: String,
    pub activity_id: String,
    pub url: String,
}