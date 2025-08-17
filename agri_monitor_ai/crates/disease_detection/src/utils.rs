//! Utilities for disease detection
//!
//! This module provides utility functions for disease detection.

use crate::models::{image_model::DiseaseImageModel, sensor_model::DiseaseSensorModel};
use crate::types::{DiseaseDetection, DiseaseType};
use common::{
    data::{ImageData, PredictionResult, SensorData},
    error::{AgriMonitorError, AgriResult},
    models::{ImageModel, SensorDataModel},
};
use std::collections::HashMap;
use std::path::Path;
use tracing::info;

/// Combined disease detection from both sensor and image data
pub async fn detect_disease(
    sensor_model: &DiseaseSensorModel,
    image_model: &DiseaseImageModel,
    sensor_data: Option<&SensorData>,
    image_data: Option<&ImageData>,
) -> AgriResult<PredictionResult<DiseaseDetection>> {
    // Check if we have at least one type of data
    if sensor_data.is_none() && image_data.is_none() {
        return Err(AgriMonitorError::ValidationError(
            "At least one of sensor data or image data must be provided".to_string(),
        ));
    }
    
    // Make predictions from available data
    let sensor_prediction = if let Some(data) = sensor_data {
        Some(sensor_model.predict(data).await?)
    } else {
        None
    };
    
    let image_prediction = if let Some(data) = image_data {
        Some(image_model.predict(data).await?)
    } else {
        None
    };
    
    // Combine predictions
    let combined_prediction = combine_predictions(sensor_prediction, image_prediction)?;
    
    Ok(combined_prediction)
}

/// Combine predictions from sensor and image models
fn combine_predictions(
    sensor_prediction: Option<PredictionResult<DiseaseDetection>>,
    image_prediction: Option<PredictionResult<DiseaseDetection>>,
) -> AgriResult<PredictionResult<DiseaseDetection>> {
    match (sensor_prediction, image_prediction) {
        (Some(sensor), Some(image)) => {
            // Both predictions available, combine them
            let timestamp = sensor.timestamp; // Use sensor timestamp
            
            // Determine if a disease is detected
            let disease_detected = sensor.prediction.disease_detected || image.prediction.disease_detected;
            
            // Use the prediction with higher confidence
            let (prediction, confidence) = if sensor.confidence >= image.confidence {
                (sensor.prediction, sensor.confidence)
            } else {
                (image.prediction, image.confidence)
            };
            
            // Combine additional info
            let mut additional_info = HashMap::new();
            additional_info.extend(sensor.additional_info);
            additional_info.extend(image.additional_info);
            additional_info.insert(
                "sensor_confidence".to_string(),
                sensor.confidence.to_string(),
            );
            additional_info.insert(
                "image_confidence".to_string(),
                image.confidence.to_string(),
            );
            
            // Create combined prediction result
            let result = PredictionResult {
                timestamp,
                prediction,
                confidence,
                additional_info,
            };
            
            Ok(result)
        }
        (Some(sensor), None) => {
            // Only sensor prediction available
            Ok(sensor)
        }
        (None, Some(image)) => {
            // Only image prediction available
            Ok(image)
        }
        (None, None) => {
            // No predictions available (should not happen due to earlier check)
            Err(AgriMonitorError::PredictionError(
                "No predictions available".to_string(),
            ))
        }
    }
}

/// Load disease detection models from files
pub async fn load_models<P: AsRef<Path> + Send + Sync>(
    sensor_model_path: Option<P>,
    image_model_path: Option<P>,
) -> AgriResult<(DiseaseSensorModel, DiseaseImageModel)> {
    // Create default models
    let mut sensor_model = DiseaseSensorModel::default();
    let mut image_model = DiseaseImageModel::default();
    
    // Load sensor model if path is provided
    if let Some(path) = sensor_model_path {
        info!("Loading sensor model from {:?}", path.as_ref());
        sensor_model.load(path).await?;
    }
    
    // Load image model if path is provided
    if let Some(path) = image_model_path {
        info!("Loading image model from {:?}", path.as_ref());
        image_model.load(path).await?;
    }
    
    Ok((sensor_model, image_model))
}

/// Save disease detection models to files
pub async fn save_models<P: AsRef<Path> + Send + Sync>(
    sensor_model: &DiseaseSensorModel,
    image_model: &DiseaseImageModel,
    sensor_model_path: P,
    image_model_path: P,
) -> AgriResult<()> {
    // Save sensor model
    info!("Saving sensor model to {:?}", sensor_model_path.as_ref());
    sensor_model.save(&sensor_model_path).await?;
    
    // Save image model
    info!("Saving image model to {:?}", image_model_path.as_ref());
    image_model.save(&image_model_path).await?;
    
    Ok(())
}

