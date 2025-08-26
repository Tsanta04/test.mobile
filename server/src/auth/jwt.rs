use crate::config::Config;
use crate::error::AppError;
use crate::models::user::{User, UserRole};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: i32,
    pub role: UserRole,
    pub exp: i64,
}

pub fn create_token(user: &User) -> Result<String, AppError> {
    let config = Config::from_env();
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(config.jwt_expiration))
        .expect("Valid timestamp")
        .timestamp();
    
    let claims = Claims {
        sub: user.username.clone(),
        user_id: user.user_id,
        role: user.role.clone(),
        exp: expiration,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Auth(format!("Failed to create token: {}", e)))
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, AppError> {
    let config = Config::from_env();
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Auth(format!("Invalid token: {}", e)))
}

