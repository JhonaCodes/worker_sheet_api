use actix_web::Responder;
use actix_web::web::{Data, Json};
use argonautica::Verifier;
use jwt::SignWithKey;
use serde_json::json;
use crate::auth::env::{date_time_epoc, hash_secret, jwt_key};
use crate::auth::models::{JwtUserInfo, LoginProfileModel, ResponseProfileModel};
use crate::helper::email_service_helper::{susses_json, un_susses};
use crate::helper::validation_helper::ValidateHelper;
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

                if user.status == "deleted" {
                    return un_susses("Usuario eliminado");
                }

                if ValidateHelper::is_valid_email(&user.email){
                   return un_susses("Al parecer, nuestro sistema detectó un error con sus datos.");
                }

                let password_clone = user.password_hash.clone();





                let is_valid = Verifier::default()
                    .with_hash(password_clone)
                    .with_password(&credentials.password_hash)
                    .with_secret_key(hash_secret())
                    .verify()
                    .unwrap();

                if is_valid {
                    let claims = JwtUserInfo::from_auth_user_model(&user, date_time_epoc(15));

                    let user_compose_json = json!({
                   "id":user.id,
                   "first_name":user.first_name,
                   "last_name":user.last_name,
                   "email":user.email,
                   "position":user.position,
                   "phone":user.phone,
                   "department":user.department,
                   "email_notification":user.email_notification,
                   "push_notification":user.push_notification,
                   "status":user.status,
                   "hash_sync": user.hash_sync,
                   "expire_at":  claims.expire_at,
                   "created_at":user.created_at,
               });

                    let user_local: JwtUserInfo = serde_json::from_str(&user_compose_json.to_string()).expect("Error profile data");

                    let token_str = claims.sign_with_key(&jwt_key()).unwrap();

                    let response_user = ResponseProfileModel{
                        jwt: token_str,
                        user: user_local,
                    };

                    susses_json(response_user)
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