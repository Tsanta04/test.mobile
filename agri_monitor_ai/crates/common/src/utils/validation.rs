//! Input validation utilities
//!
//! This module provides utility functions for validating input data.

use crate::data::{ImageData, SensorData};
use crate::error::{AgriMonitorError, AgriResult};

/// Validate sensor data
pub fn validate_sensor_data(data: &SensorData) -> AgriResult<()> {
    // Check if at least one sensor reading is present
    if data.values.is_empty() {
        return Err(AgriMonitorError::ValidationError(
            "Sensor data must contain at least one reading".to_string(),
        ));
    }
    
    // Validate location
    if !(-90.0..=90.0).contains(&data.location.latitude) {
        return Err(AgriMonitorError::ValidationError(
            format!("Latitude must be between -90 and 90, got {}", data.location.latitude),
        ));
    }
    
    if !(-180.0..=180.0).contains(&data.location.longitude) {
        return Err(AgriMonitorError::ValidationError(
            format!("Longitude must be between -180 and 180, got {}", data.location.longitude),
        ));
    }
    
    // Validate specific sensor values if present
    if let Some(moisture) = data.get_value("soil_moisture") {
        if let Ok(moisture) = moisture.parse::<f64>() {
            if !(0.0..=100.0).contains(&moisture) {
                return Err(AgriMonitorError::ValidationError(
                    format!("Soil moisture must be between 0 and 100, got {}", moisture),
                ));
            }
        }
    }
    
    if let Some(humidity) = data.get_value("air_humidity") {
        if let Ok(humidity) = humidity.parse::<f64>() {
            if !(0.0..=100.0).contains(&humidity) {
                return Err(AgriMonitorError::ValidationError(
                    format!("Air humidity must be between 0 and 100, got {}", humidity),
                ));
            }
        }
    }
    
    if let Some(ph) = data.get_value("soil_ph") {
        if let Ok(ph) = ph.parse::<f64>() {
            if !(0.0..=14.0).contains(&ph) {
                return Err(AgriMonitorError::ValidationError(
                    format!("Soil pH must be between 0 and 14, got {}", ph),
                ));
            }
        }
    }
    
    Ok(())
}

/// Validate image data
pub fn validate_image_data(data: &ImageData) -> AgriResult<()> {
    // Check if image data exists
    if data.image.is_empty() {
        return Err(AgriMonitorError::ValidationError(
            "Image data is empty".to_string(),
        ));
    }
    
    // Validate location
    if !(-90.0..=90.0).contains(&data.location.latitude) {
        return Err(AgriMonitorError::ValidationError(
            format!("Latitude must be between -90 and 90, got {}", data.location.latitude),
        ));
    }
    
    if !(-180.0..=180.0).contains(&data.location.longitude) {
        return Err(AgriMonitorError::ValidationError(
            format!("Longitude must be between -180 and 180, got {}", data.location.longitude),
        ));
    }
    
    // Validate image type
    if data.image_type.is_empty() {
        return Err(AgriMonitorError::ValidationError(
            "Image type is empty".to_string(),
        ));
    }
    
    Ok(())
}
