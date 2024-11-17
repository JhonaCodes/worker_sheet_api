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

#[derive(Deserialize)]
pub struct UserFilters {
    pub department: Option<String>,
    pub position: Option<String>,
    pub status: Option<String>,
    pub created_from: Option<NaiveDateTime>,
    pub created_to: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct UpdateUserStatus {
    pub status: String,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct UpdateUserNotifications {
    pub email_notification: bool,
    pub push_notification: bool,
    pub auto_sync: bool,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct UpdateUserPassword {
    pub current_password: String,
    pub new_password: String,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct UserActivitiesStats {
    pub total_activities: i64,
    pub pending_activities: i64,
    pub completed_activities: i64,
    pub high_risk_activities: i64,
    pub latest_activity_date: Option<NaiveDateTime>,
}