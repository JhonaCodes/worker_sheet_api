use std::env;
use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::{bearer, AuthenticationError};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use crate::auth::models::JwtUserInfo;

pub async fn validate_jwt(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token_string = credentials.token();

    let claims: Result<JwtUserInfo, &str> = token_string
        .verify_with_key(&jwt_key())
        .map_err(|_| "Invalid token");

    let config = req.app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .scope("Error on create validate request.");

    match claims {
        Ok(value) => {
            let is_jwt_expired = value.expire_at < date_time_epoc(0);

            if is_jwt_expired {
                log::error!("IS JWT EXPIRED? {}", is_jwt_expired);
                Err((AuthenticationError::from(config).into(), req))
            } else {
                req.extensions_mut().insert(value);
                Ok(req)
            }
        }

        Err(err) => {
            log::error!("{}", err);
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

pub fn jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set!")
}

/// Create a hash secret return on one place
pub fn hash_secret() -> String {
    return env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
}

pub fn jwt_key()->Hmac<Sha256>{
    return Hmac::new_from_slice(jwt_secret().as_bytes()).expect("Failed on cryptography");
}

pub fn date_time_epoc(days: i64) -> i64{

    // Obtener la fecha y hora actual en UTC
    let date_now = Utc::now();

    // Sumar los días a la fecha actual
    let expiration = match days {
        0 | 1 => date_now,
        _ => date_now + Duration::days(days),
    };

    // Devolver la marca de tiempo (timestamp) de la fecha de expiración
    return expiration.timestamp_millis();
}

/// Api key for companies process
pub fn api_key() -> String {
    return env::var("API_KEY").expect("HASH_SECRET must be set!");
}
