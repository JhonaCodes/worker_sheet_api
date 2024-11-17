use actix_web::{get, web::Json};

use crate::auth::models::UserInfo;
use uuid::Uuid;

#[get("/user-test")]
pub async fn user_data() -> Result<Json<UserInfo>, actix_web::Error> {
    let user = UserInfo {
        id: Uuid::new_v4(),
        name: String::from("John Doe"),
        email: String::from("john@example.com"),
    };

    Ok(Json(user))
}
