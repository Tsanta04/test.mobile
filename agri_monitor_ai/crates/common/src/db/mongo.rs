//! MongoDB connection and operations
//!
//! This module provides functionality to connect to MongoDB and perform operations
//! for storing and retrieving images and their metadata.

use crate::error::{AgriMonitorError, AgriResult};
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection, Database,
};
use std::sync::Arc;

/// MongoDB client wrapper
#[derive(Clone)]
pub struct MongoClient {
    db: Arc<Database>,
}

impl MongoClient {
    /// Create a new MongoDB client
    pub async fn new(uri: &str, db_name: &str) -> AgriResult<Self> {
        let client_options = ClientOptions::parse(uri)
            .await
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to parse MongoDB URI: {}", e)))?;
        
        let client = Client::with_options(client_options)
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to create MongoDB client: {}", e)))?;
        
        let db = client.database(db_name);
        
        Ok(Self {
            db: Arc::new(db),
        })
    }
    
    /// Get a collection from the database
    pub fn collection<T>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }
    
    /// Insert a document into a collection
    pub async fn insert_one(&self, collection_name: &str, document: Document) -> AgriResult<String> {
        let collection = self.db.collection::<Document>(collection_name);
        
        let result = collection
            .insert_one(document, None)
            .await
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to insert document: {}", e)))?;
        
        Ok(result
            .inserted_id
            .as_object_id()
            .map(|id| id.to_hex())
            .unwrap_or_default())
    }
    
    /// Find documents in a collection
    pub async fn find(&self, collection_name: &str, filter: Document) -> AgriResult<Vec<Document>> {
        let collection = self.db.collection::<Document>(collection_name);
        
        let cursor = collection
            .find(filter, None)
            .await
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to find documents: {}", e)))?;
        
        let documents = cursor
            .try_collect()
            .await
            .map_err(|e| AgriMonitorError::DatabaseError(format!("Failed to collect documents: {}", e)))?;
        
        Ok(documents)
    }
}

/// Initialize MongoDB connection from environment variables
pub async fn init_mongo_client() -> AgriResult<MongoClient> {
    // In a real application, these would be loaded from environment variables
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let db_name = std::env::var("MONGODB_DB").unwrap_or_else(|_| "agri_monitor".to_string());
    
    MongoClient::new(&uri, &db_name).await
}

