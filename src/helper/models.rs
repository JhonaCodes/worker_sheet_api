use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Receibe error and internall decode error and return personalized message
#[derive(Deserialize, FromRow, Serialize)]
pub struct MessageResponse {
    pub title: String,
    pub content: String,
}


impl MessageResponse{
    pub fn error(content: String) ->MessageResponse{
        MessageResponse{title:String::from("Error"),content }
    }

    pub fn susses(content: String)->MessageResponse{
        MessageResponse{title:String::from("Susses"),content}
    }
}
