//! Common data structures for input and output data
//!
//! This module defines common data structures used across all prediction modules.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Sensor data input for raw data predictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    /// Timestamp of the sensor reading
    pub timestamp: DateTime<Utc>,
    
    /// Location of the sensor (latitude, longitude)
    pub location: GeoLocation,
    
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
    pub additional_data: std::collections::HashMap<String, f32>,
}

/// Image data input for image-based predictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    /// Timestamp when the image was captured
    pub timestamp: DateTime<Utc>,
    
    /// Location where the image was captured
    pub location: GeoLocation,
    
    /// Path to the image file
    pub image_path: PathBuf,
    
    /// Image type (RGB, multispectral, etc.)
    pub image_type: ImageType,
    
    /// Image resolution (width, height)
    pub resolution: (u32, u32),
    
    /// Additional metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, String>,
}

/// Geographic location
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoLocation {
    /// Latitude in decimal degrees
    pub latitude: f64,
    
    /// Longitude in decimal degrees
    pub longitude: f64,
    
    /// Altitude in meters (optional)
    pub altitude: Option<f64>,
}

/// Image type enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageType {
    /// RGB image (standard color)
    RGB,
    
    /// Multispectral image
    Multispectral,
    
    /// Thermal image
    Thermal,
    
    /// Satellite image
    Satellite,
    
    /// Drone image
    Drone,
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
    pub additional_info: std::collections::HashMap<String, String>,
}

/// Alert level enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertLevel {
    /// No alert
    None,
    
    /// Low alert level
    Low,
    
    /// Medium alert level
    Medium,
    
    /// High alert level
    High,
    
    /// Critical alert level
    Critical,
}

