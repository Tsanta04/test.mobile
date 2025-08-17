//! Common data structures for input and output data
//!
//! This module defines common data structures used across all prediction modules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Suppression de l'import inutilisé

/// Sensor data input for raw data predictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    /// Timestamp of the sensor reading
    pub timestamp: DateTime<Utc>,
    
    /// Location of the sensor (latitude, longitude)
    pub location: GeoLocation,
    
    /// Values from various sensors as key-value pairs
    pub values: HashMap<String, String>,
}

impl SensorData {
    /// Get a value from the sensor data
    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }
}

/// Image data input for image-based predictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    /// Timestamp when the image was captured
    pub timestamp: DateTime<Utc>,
    
    /// Location where the image was captured
    pub location: GeoLocation,
    
    /// Raw image data
    pub image: Vec<u8>,
    
    /// Image type (format)
    pub image_type: String,
}

/// Geographic location
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoLocation {
    /// Latitude in decimal degrees
    pub latitude: f64,
    
    /// Longitude in decimal degrees
    pub longitude: f64,
}

/// Prediction result with confidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult<T> {
    /// Timestamp of the prediction
    pub timestamp: DateTime<Utc>,
    
    /// Predicted value
    pub prediction: T,
    
    /// Confidence level (0.0-1.0)
    pub confidence: f32,
    
    /// Additional information about the prediction
    pub additional_info: HashMap<String, String>,
}
