use actix_web::{get, web::Json};

use uuid::Uuid;
use crate::auth::models::UserInfo;

#[get("/")]
pub async fn user_data() -> Result<Json<UserInfo>, actix_web::Error> {
    let user = UserInfo {
        id: Uuid::new_v4(),
        name: String::from("John Doe"),
        email: String::from("john@example.com"),
    };
    
    
    Ok(Json(user))
 
}