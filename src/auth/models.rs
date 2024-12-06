use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::user::models::UserModel;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct JwtUserInfo {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone:String,
    pub position:String,
    pub department: String,
    pub email_notification: bool,
    pub push_notification: bool,
    pub status: String,
    pub hash_sync: String,
    pub expire_at: i64,
    pub created_at: Option<NaiveDateTime>
}


/// Implementation for use a [AuthProfileModel] to create JWT from [JwtProfileModel].
impl JwtUserInfo {
    pub fn from_auth_user_model(user: &UserModel, expire: i64) -> JwtUserInfo {
        JwtUserInfo {
            id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            position: user.position.clone(),
            department: user.department.clone(),
            email_notification:user.email_notification,
            push_notification: user.push_notification,
            phone:user.phone.clone(),
            status: user.status.clone(),
            hash_sync: user.hash_sync.clone().unwrap(),
            expire_at: expire,
            created_at: None,
        }
    }
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct LoginProfileModel {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct ResponseProfileModel {
    pub jwt: String,
    pub user: JwtUserInfo
}