//! PostgreSQL connection and operations using Diesel ORM
//!
//! This module provides functionality to connect to PostgreSQL and perform operations
//! for relational data like economic analysis.

use crate::error::{AgriMonitorError, AgriResult};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use std::sync::Arc;

/// Type alias for a PostgreSQL connection pool
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// Type alias for a pooled PostgreSQL connection
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// PostgreSQL client wrapper
#[derive(Clone)]
pub struct PostgresClient {
    pool: Arc<PgPool>,
}

impl PostgresClient {
    /// Create a new PostgreSQL client
    pub fn new(database_url: &str) -> AgriResult<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to create connection pool: {}", e)))?;
        
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
    
    /// Get a connection from the pool
    pub fn get_connection(&self) -> AgriResult<PgPooledConnection> {
        self.pool
            .get()
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to get database connection: {}", e)))
    }
}

/// Initialize PostgreSQL connection from environment variables
pub fn init_postgres_client() -> AgriResult<PostgresClient> {
    // In a real application, these would be loaded from environment variables
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/agri_monitor".to_string());
    
    PostgresClient::new(&database_url)
}

