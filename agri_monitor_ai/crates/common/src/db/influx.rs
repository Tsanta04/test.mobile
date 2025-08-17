//! InfluxDB connection and operations
//!
//! This module provides functionality to connect to InfluxDB and perform operations
//! for time series data like sensor readings.

use crate::error::{AgriMonitorError, AgriResult};
use influxdb::{Client, InfluxDbWriteable};
use std::sync::Arc;
use tokio::sync::Mutex;

/// InfluxDB client wrapper
#[derive(Clone)]
pub struct InfluxClient {
    client: Arc<Mutex<Client>>,
}

impl InfluxClient {
    /// Create a new InfluxDB client
    pub fn new(url: &str, org: &str, bucket: &str, token: &str) -> Self {
        let client = Client::new(url, bucket)
            .with_token(token)
            .with_org(org);
        
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }
    
    /// Write data to InfluxDB
    pub async fn write<T: InfluxDbWriteable>(&self, data: T) -> AgriResult<()> {
        let client = self.client.lock().await;
        client
            .query(data.into_query("measurement"))
            .await
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to write to InfluxDB: {}", e)))?;
        
        Ok(())
    }
    
    /// Query data from InfluxDB using Flux query language
    pub async fn query(&self, query: &str) -> AgriResult<String> {
        let client = self.client.lock().await;
        let response = client
            .query(query)
            .await
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to query InfluxDB: {}", e)))?;
        
        Ok(response)
    }
}

/// Initialize InfluxDB connection from environment variables
pub fn init_influx_client() -> AgriResult<InfluxClient> {
    // In a real application, these would be loaded from environment variables
    // using dotenv or similar
    let url = std::env::var("INFLUXDB_URL").unwrap_or_else(|_| "http://localhost:8086".to_string());
    let org = std::env::var("INFLUXDB_ORG").unwrap_or_else(|_| "agri_monitor".to_string());
    let bucket = std::env::var("INFLUXDB_BUCKET").unwrap_or_else(|_| "sensor_data".to_string());
    let token = std::env::var("INFLUXDB_TOKEN").unwrap_or_else(|_| "my-token".to_string());
    
    Ok(InfluxClient::new(&url, &org, &bucket, &token))
}

