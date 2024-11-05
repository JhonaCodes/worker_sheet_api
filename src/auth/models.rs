use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserInfo{
    pub id: Uuid,
    pub name:String,
    pub email:String
}