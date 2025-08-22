//! Disease detection handlers
//!
//! This module provides handlers for disease detection endpoints.

use crate::models::{ApiResponse, DiseaseDetectionImageRequest, DiseaseDetectionResponse, DiseaseDetectionSensorRequest};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use common::data::{GeoLocation, SensorData}; // ImageData, ImageType retirés car non utilisés
use common::models::SensorDataModel;
use disease_detection::{
    models::{image_model::DiseaseImageModel, sensor_model::DiseaseSensorModel},
    types::DiseaseType,
    utils,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error; // info retiré car non utilisé

/// Shared state for disease detection handlers
pub struct DiseaseDetectionState {
    /// Sensor model for disease detection
    pub sensor_model: Arc<Mutex<DiseaseSensorModel>>,
    
    /// Image model for disease detection
    pub image_model: Arc<Mutex<DiseaseImageModel>>,
}

impl DiseaseDetectionState {
    /// Create a new disease detection state
    pub async fn new(models_dir: &str) -> Self {
        // Create models directory if it doesn't exist
        let models_dir = PathBuf::from(models_dir);
        if !models_dir.exists() {
            std::fs::create_dir_all(&models_dir).unwrap_or_else(|e| {
                error!("Failed to create models directory: {}", e);
            });
        }
        
        // Paths to model files
        let sensor_model_path = models_dir.join("disease_sensor_model.bin");
        let image_model_path = models_dir.join("disease_image_model.bin");
        
        // Load models
        let (sensor_model, image_model) = match utils::load_models(
            Some(sensor_model_path),
            Some(image_model_path),
        ).await {
            Ok((sensor_model, image_model)) => (sensor_model, image_model),
            Err(e) => {
                error!("Failed to load models: {}", e);
                (DiseaseSensorModel::default(), DiseaseImageModel::default())
            }
        };
        
        Self {
            sensor_model: Arc::new(Mutex::new(sensor_model)),
            image_model: Arc::new(Mutex::new(image_model)),
        }
    }
}

/// Helper function to convert API request to SensorData
fn convert_additional_data_to_values(request: &DiseaseDetectionSensorRequest) -> HashMap<String, String> {
    let mut values = HashMap::new();
    
    // Add all sensor values to the map
    if let Some(val) = request.soil_moisture {
        values.insert("soil_moisture".to_string(), val.to_string());
    }
    if let Some(val) = request.air_humidity {
        values.insert("air_humidity".to_string(), val.to_string());
    }
    if let Some(val) = request.temperature {
        values.insert("temperature".to_string(), val.to_string());
    }
    if let Some(val) = request.soil_ph {
        values.insert("soil_ph".to_string(), val.to_string());
    }
    if let Some(val) = request.nitrogen {
        values.insert("nitrogen".to_string(), val.to_string());
    }
    if let Some(val) = request.phosphorus {
        values.insert("phosphorus".to_string(), val.to_string());
    }
    if let Some(val) = request.potassium {
        values.insert("potassium".to_string(), val.to_string());
    }
    if let Some(val) = request.co2 {
        values.insert("co2".to_string(), val.to_string());
    }
    if let Some(val) = request.pm25 {
        values.insert("pm25".to_string(), val.to_string());
    }
    if let Some(val) = request.pm10 {
        values.insert("pm10".to_string(), val.to_string());
    }
    if let Some(val) = request.wind_speed {
        values.insert("wind_speed".to_string(), val.to_string());
    }
    if let Some(val) = request.rainfall {
        values.insert("rainfall".to_string(), val.to_string());
    }
    if let Some(val) = request.solar_radiation {
        values.insert("solar_radiation".to_string(), val.to_string());
    }
    
    // Add additional data
    if let Some(additional_data) = &request.additional_data {
        for (key, value) in additional_data {
            values.insert(key.clone(), value.to_string());
        }
    }
    
    values
}

/// Handler for disease detection with sensor data
pub async fn detect_disease_sensor(
    state: web::Data<DiseaseDetectionState>,
    request: web::Json<DiseaseDetectionSensorRequest>,
) -> HttpResponse {
    let request = request.into_inner();
    
    // Create sensor data from request
    let sensor_data = SensorData {
        timestamp: request.timestamp.unwrap_or_else(Utc::now),
        location: GeoLocation {
            latitude: request.latitude,
            longitude: request.longitude,
        },
        values: convert_additional_data_to_values(&request),
    };
    
    // Get models
    let sensor_model = state.sensor_model.lock().await;
    
    // Predict disease
    match sensor_model.predict(&sensor_data).await {
        Ok(result) => {
            // Convert to API response
            let response = DiseaseDetectionResponse {
                timestamp: result.timestamp,
                disease_detected: result.prediction.disease_detected,
                disease_type: match result.prediction.disease_type {
                    Some(DiseaseType::Bacterial(_)) => Some("bacterial".to_string()),
                    Some(DiseaseType::Fungal(_)) => Some("fungal".to_string()),
                    Some(DiseaseType::Viral(_)) => Some("viral".to_string()),
                    Some(DiseaseType::Pest(_)) => Some("pest".to_string()),
                    Some(DiseaseType::NutrientDeficiency(_)) => Some("nutrient_deficiency".to_string()),
                    Some(DiseaseType::Other(s)) => Some(format!("other: {}", s)),
                    None => None,
                },
                severity: result.prediction.severity,
                affected_area: result.prediction.affected_area,
                confidence: result.confidence,
                recommendations: result.prediction.recommendations,
                additional_info: result.additional_info,
            };
            
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => {
            error!("Failed to predict disease: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string()))
        }
    }
}

/// Handler for disease detection with image data
pub async fn detect_disease_image(
    _state: web::Data<DiseaseDetectionState>,
    _request: web::Json<DiseaseDetectionImageRequest>,
) -> HttpResponse {
    // This is a placeholder for the image-based disease detection
    // In a real implementation, this would process the image and use the image model
    
    HttpResponse::NotImplemented().json(ApiResponse::<()>::error(
        "Image-based disease detection not implemented yet".to_string(),
    ))
}
