//! API models
//!
//! This module defines the data models used in the API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    
    /// Response data
    pub data: Option<T>,
    
    /// Error message
    pub error: Option<String>,
    
    /// Timestamp of the response
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }
    
    /// Create an error response
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

/// Disease detection request with sensor data
#[derive(Debug, Serialize, Deserialize)]
pub struct DiseaseDetectionSensorRequest {
    /// Timestamp of the sensor reading
    pub timestamp: Option<DateTime<Utc>>,
    
    /// Location of the sensor (latitude, longitude)
    pub latitude: f64,
    
    /// Location of the sensor (latitude, longitude)
    pub longitude: f64,
    
    /// Soil moisture percentage (0-100)
    pub soil_moisture: Option<f32>,
    
    /// Air humidity percentage (0-100)
    pub air_humidity: Option<f32>,
    
    /// Temperature in Celsius
    pub temperature: Option<f32>,
    
    /// Soil pH level (0-14)
    pub soil_ph: Option<f32>,
    
    /// Nitrogen level in soil (ppm)
    pub nitrogen: Option<f32>,
    
    /// Phosphorus level in soil (ppm)
    pub phosphorus: Option<f32>,
    
    /// Potassium level in soil (ppm)
    pub potassium: Option<f32>,
    
    /// Carbon dioxide level in air (ppm)
    pub co2: Option<f32>,
    
    /// PM2.5 particulate matter in air (μg/m³)
    pub pm25: Option<f32>,
    
    /// PM10 particulate matter in air (μg/m³)
    pub pm10: Option<f32>,
    
    /// Wind speed (m/s)
    pub wind_speed: Option<f32>,
    
    /// Rainfall amount (mm)
    pub rainfall: Option<f32>,
    
    /// Solar radiation (W/m²)
    pub solar_radiation: Option<f32>,
    
    /// Additional sensor data as key-value pairs
    pub additional_data: Option<HashMap<String, f32>>,
}

/// Disease detection request with image data
#[derive(Debug, Serialize, Deserialize)]
pub struct DiseaseDetectionImageRequest {
    /// Timestamp of the image
    pub timestamp: Option<DateTime<Utc>>,
    
    /// Location of the image (latitude, longitude)
    pub latitude: f64,
    
    /// Location of the image (latitude, longitude)
    pub longitude: f64,
    
    /// Base64-encoded image data
    pub image_data: String,
    
    /// Image type (RGB, multispectral, etc.)
    pub image_type: String,
    
    /// Additional metadata as key-value pairs
    pub metadata: Option<HashMap<String, String>>,
}

/// Disease detection response
#[derive(Debug, Serialize, Deserialize)]
pub struct DiseaseDetectionResponse {
    /// Whether a disease was detected
    pub disease_detected: bool,
    
    /// The type of disease detected
    pub disease_type: Option<String>,
    
    /// The severity of the disease (0.0-1.0)
    pub severity: f32,
    
    /// The affected area (percentage of plant affected)
    pub affected_area: Option<f32>,
    
    /// Confidence level of the prediction (0.0-1.0)
    pub confidence: f32,
    
    /// Recommended actions to take
    pub recommendations: Vec<String>,
    
    /// Additional information about the disease
    pub additional_info: HashMap<String, String>,
    
    /// Timestamp of the prediction
    pub timestamp: DateTime<Utc>,
}

