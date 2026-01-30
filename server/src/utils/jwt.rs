use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::errors::{AppError, AppResult};

/// Structure des claims JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user_id
    pub username: String,
    pub exp: i64, // expiration timestamp
}

/// Crée un token JWT pour un utilisateur
pub fn create_token(user_id: i32, username: &str) -> AppResult<String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_change_me".to_string());
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AppError::InternalServerError)
}

/// Vérifie et décode un token JWT
pub fn verify_token(token: &str) -> AppResult<Claims> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_change_me".to_string());
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::InvalidToken)
}
