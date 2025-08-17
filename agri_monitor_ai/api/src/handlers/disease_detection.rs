//! Disease detection handlers
//!
//! This module provides handlers for disease detection endpoints.

use crate::models::{ApiResponse, DiseaseDetectionImageRequest, DiseaseDetectionResponse, DiseaseDetectionSensorRequest};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use common::data::{GeoLocation, ImageData, ImageType, SensorData};
use disease_detection::{
    models::{image_model::DiseaseImageModel, sensor_model::DiseaseSensorModel},
    types::DiseaseType,
    utils,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

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
                // Create default models
                (DiseaseSensorModel::default(), DiseaseImageModel::default())
            }
        };
        
        Self {
            sensor_model: Arc::new(Mutex::new(sensor_model)),
            image_model: Arc::new(Mutex::new(image_model)),
        }
    }
}

/// Handler for disease detection with sensor data
pub async fn detect_disease_sensor(
    state: web::Data<DiseaseDetectionState>,
    request: web::Json<DiseaseDetectionSensorRequest>,
) -> HttpResponse {
    let request = request.into_inner();
    
    // Convert request to SensorData
    let sensor_data = SensorData {
        timestamp: request.timestamp.unwrap_or_else(Utc::now),
        location: GeoLocation {
            latitude: request.latitude,
            longitude: request.longitude,
            altitude: None,
        },
        soil_moisture: request.soil_moisture,
        air_humidity: request.air_humidity,
        temperature: request.temperature,
        soil_ph: request.soil_ph,
        nitrogen: request.nitrogen,
        phosphorus: request.phosphorus,
        potassium: request.potassium,
        co2: request.co2,
        pm25: request.pm25,
        pm10: request.pm10,
        wind_speed: request.wind_speed,
        rainfall: request.rainfall,
        solar_radiation: request.solar_radiation,
        additional_data: request.additional_data.unwrap_or_default(),
    };
    
    // Get models
    let sensor_model = state.sensor_model.lock().await;
    let image_model = state.image_model.lock().await;
    
    // Detect disease
    match utils::detect_disease(&sensor_model, &image_model, Some(&sensor_data), None).await {
        Ok(result) => {
            // Convert result to response
            let response = DiseaseDetectionResponse {
                disease_detected: result.prediction.disease_detected,
                disease_type: result.prediction.disease_type.map(|d| format!("{:?}", d)),
                severity: result.prediction.severity,
                affected_area: result.prediction.affected_area,
                confidence: result.confidence,
                recommendations: result.prediction.recommendations,
                additional_info: result.additional_info,
                timestamp: result.timestamp,
            };
            
            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        Err(e) => {
            error!("Failed to detect disease: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(e.to_string()))
        }
    }
}

/// Handler for disease detection with image data
pub async fn detect_disease_image(
    state: web::Data<DiseaseDetectionState>,
    request: web::Json<DiseaseDetectionImageRequest>,
) -> HttpResponse {
    let request = request.into_inner();
    
    // In a real implementation, we would:
    // 1. Decode the base64 image data
    // 2. Save it to a temporary file
    // 3. Create an ImageData struct with the file path
    
    // For this example, we'll just return a mock response
    let response = DiseaseDetectionResponse {
        disease_detected: true,
        disease_type: Some("Fungal(PowderyMildew)".to_string()),
        severity: 0.8,
        affected_area: Some(0.3),
        confidence: 0.9,
        recommendations: vec![
            "Remove and destroy infected plant parts".to_string(),
            "Apply appropriate fungicide".to_string(),
            "Ensure good air circulation".to_string(),
        ],
        additional_info: HashMap::new(),
        timestamp: Utc::now(),
    };
    
    HttpResponse::Ok().json(ApiResponse::success(response))
}

/// Handler for health check
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("Disease detection service is healthy"))
}

