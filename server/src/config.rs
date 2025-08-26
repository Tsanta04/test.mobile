use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string())
            .parse::<i64>()
            .expect("JWT_EXPIRATION must be a valid number");

        Self {
            database_url,
            jwt_secret,
            jwt_expiration,
        }
    }
}

// Constants for the application
pub const API_VERSION: &str = "v1";
pub const API_PREFIX: &str = "/api";

