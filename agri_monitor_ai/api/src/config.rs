//! Configuration for the API
//!
//! This module provides configuration for the API server.

/// API server configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// Server host
    pub host: String,
    
    /// Server port
    pub port: u16,
    
    /// Database URL for PostgreSQL
    pub database_url: String,
    
    /// InfluxDB URL
    pub influxdb_url: String,
    
    /// InfluxDB organization
    pub influxdb_org: String,
    
    /// InfluxDB bucket
    pub influxdb_bucket: String,
    
    /// InfluxDB token
    pub influxdb_token: String,
    
    /// MongoDB URI
    pub mongodb_uri: String,
    
    /// MongoDB database name
    pub mongodb_db: String,
    
    /// Path to models directory
    pub models_dir: String,
}

impl Config {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/agri_monitor".to_string()),
            influxdb_url: std::env::var("INFLUXDB_URL")
                .unwrap_or_else(|_| "http://localhost:8086".to_string()),
            influxdb_org: std::env::var("INFLUXDB_ORG")
                .unwrap_or_else(|_| "agri_monitor".to_string()),
            influxdb_bucket: std::env::var("INFLUXDB_BUCKET")
                .unwrap_or_else(|_| "sensor_data".to_string()),
            influxdb_token: std::env::var("INFLUXDB_TOKEN")
                .unwrap_or_else(|_| "my-token".to_string()),
            mongodb_uri: std::env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            mongodb_db: std::env::var("MONGODB_DB")
                .unwrap_or_else(|_| "agri_monitor".to_string()),
            models_dir: std::env::var("MODELS_DIR")
                .unwrap_or_else(|_| "./models".to_string()),
        }
    }
}

