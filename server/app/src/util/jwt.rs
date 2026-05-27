use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use serde::{Deserialize, Serialize};

use crate::util::app_error::AppResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    user_id: i32,
    user_name: String,
}

pub fn create_token(user_id: i32, user_name: String) -> AppResult<String> {
    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        user_id,
        user_name,
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}

pub fn is_valid_token(token: &str) -> AppResult<bool> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    ) {
        Ok(_) => Ok(true),
        Err(e) => {
            let src = e.clone();
            match e.into_kind() {
                ErrorKind::InvalidToken | ErrorKind::ExpiredSignature => Ok(false),
                _ => {
                    println!("Error: {}", src);
                    Ok(false)
                }
            }
        }
    }
}
