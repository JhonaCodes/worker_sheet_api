use actix_web::Responder;
use actix_web::web::{Data, Json};
use argonautica::Verifier;
use jwt::SignWithKey;
use crate::auth::env::{date_time_epoc, hash_secret, jwt_key};
use crate::auth::models::{JwtUserInfo, LoginProfileModel, ResponseProfileModel};
use crate::helper::response::{susses_json, un_susses};
use crate::model::AppState;
use crate::user::models::UserModel;

pub struct AuthRepository;

impl AuthRepository {

    pub async fn basic_auth(state: Data<AppState>, credentials: Json<LoginProfileModel>) -> impl Responder {
        match sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE email = $1")
            .bind(&credentials.email)
            .fetch_one(&state.db).await
        {
            Ok(user) => {
                let password_clone = user.password_hash.clone();

                let is_valid = Verifier::default()
                    .with_hash(password_clone)
                    .with_password(&credentials.password_hash)
                    .with_secret_key(hash_secret())
                    .verify()
                    .unwrap();

                if is_valid {

                    let claims = JwtUserInfo::from_auth_user_model(&user, date_time_epoc(15));

                    let user_data_json = serde_json::to_string(&user).expect("Error profile data json");

                    let user_local: JwtUserInfo = serde_json::from_str(&user_data_json).expect("Error profile data");
                    let token_str = claims.sign_with_key(&jwt_key()).unwrap();

                    let response_user = ResponseProfileModel{
                        jwt: token_str,
                        user: user_local
                    };

                    susses_json(response_user )

                } else {
                    un_susses("Incorrect username or password")
                }
            }
            Err(_) =>  un_susses("Incorrect username or password")
        }
    }


    pub async fn check_jwt() -> impl Responder {
        return susses_json(true);
    }
}