use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub server_addr: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Config {
            database_url,
            server_addr,
            jwt_secret,
        }
    }
}

