//! Input validation utilities
//!
//! This module provides utility functions for validating input data.

use crate::data::{ImageData, SensorData};
use crate::error::{AgriMonitorError, AgriResult};

/// Validate sensor data
pub fn validate_sensor_data(data: &SensorData) -> AgriResult<()> {
    // Check if at least one sensor reading is present
    if data.soil_moisture.is_none()
        && data.air_humidity.is_none()
        && data.temperature.is_none()
        && data.soil_ph.is_none()
        && data.nitrogen.is_none()
        && data.phosphorus.is_none()
        && data.potassium.is_none()
        && data.co2.is_none()
        && data.pm25.is_none()
        && data.pm10.is_none()
        && data.wind_speed.is_none()
        && data.rainfall.is_none()
        && data.solar_radiation.is_none()
        && data.additional_data.is_empty()
    {
        return Err(AgriMonitorError::ValidationError(
            "Sensor data must contain at least one reading".to_string(),
        ));
    }
    
    // Validate ranges for sensor readings
    if let Some(moisture) = data.soil_moisture {
        if !(0.0..=100.0).contains(&moisture) {
            return Err(AgriMonitorError::ValidationError(
                format!("Soil moisture must be between 0 and 100, got {}", moisture),
            ));
        }
    }
    
    if let Some(humidity) = data.air_humidity {
        if !(0.0..=100.0).contains(&humidity) {
            return Err(AgriMonitorError::ValidationError(
                format!("Air humidity must be between 0 and 100, got {}", humidity),
            ));
        }
    }
    
    if let Some(ph) = data.soil_ph {
        if !(0.0..=14.0).contains(&ph) {
            return Err(AgriMonitorError::ValidationError(
                format!("Soil pH must be between 0 and 14, got {}", ph),
            ));
        }
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
    
    Ok(())
}

/// Validate image data
pub fn validate_image_data(data: &ImageData) -> AgriResult<()> {
    // Check if image file exists
    if !data.image_path.exists() {
        return Err(AgriMonitorError::ValidationError(
            format!("Image file does not exist: {:?}", data.image_path),
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
    
    // Validate resolution
    if data.resolution.0 == 0 || data.resolution.1 == 0 {
        return Err(AgriMonitorError::ValidationError(
            format!("Image resolution must be positive, got {:?}", data.resolution),
        ));
    }
    
    Ok(())
}

