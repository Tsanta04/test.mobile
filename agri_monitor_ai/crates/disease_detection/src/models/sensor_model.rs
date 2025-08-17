//! Sensor-based disease detection model
//!
//! This module implements a Random Forest model for detecting plant diseases
//! using sensor data (soil moisture, temperature, etc.).

use crate::types::{DiseaseDetection, DiseaseType};
use async_trait::async_trait;
use common::{
    data::{PredictionResult, SensorData},
    error::{AgriMonitorError, AgriResult},
    models::SensorDataModel,
};
use linfa::prelude::*;
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::{Array1, Array2, Axis};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Random Forest model for disease detection from sensor data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseSensorModel {
    /// The trained model (serialized)
    #[serde(skip)]
    model: Option<DecisionTree<f32, usize>>,
    
    /// Path to the model file
    model_path: Option<PathBuf>,
    
    /// Feature names used by the model
    feature_names: Vec<String>,
    
    /// Disease types that can be detected
    disease_types: Vec<DiseaseType>,
    
    /// Confidence threshold for disease detection
    confidence_threshold: f32,
}

impl Default for DiseaseSensorModel {
    fn default() -> Self {
        Self {
            model: None,
            model_path: None,
            feature_names: vec![
                "soil_moisture".to_string(),
                "air_humidity".to_string(),
                "temperature".to_string(),
                "soil_ph".to_string(),
                "nitrogen".to_string(),
                "phosphorus".to_string(),
                "potassium".to_string(),
            ],
            disease_types: vec![],
            confidence_threshold: 0.7,
        }
    }
}

impl DiseaseSensorModel {
    /// Create a new disease sensor model
    pub fn new(
        feature_names: Vec<String>,
        disease_types: Vec<DiseaseType>,
        confidence_threshold: f32,
    ) -> Self {
        Self {
            model: None,
            model_path: None,
            feature_names,
            disease_types,
            confidence_threshold,
        }
    }
    
    /// Extract features from sensor data
    fn extract_features(&self, data: &SensorData) -> AgriResult<Array1<f32>> {
        let mut features = Vec::with_capacity(self.feature_names.len());
        
        for feature_name in &self.feature_names {
            let value = match feature_name.as_str() {
                "soil_moisture" => data.soil_moisture.unwrap_or(0.0),
                "air_humidity" => data.air_humidity.unwrap_or(0.0),
                "temperature" => data.temperature.unwrap_or(0.0),
                "soil_ph" => data.soil_ph.unwrap_or(0.0),
                "nitrogen" => data.nitrogen.unwrap_or(0.0),
                "phosphorus" => data.phosphorus.unwrap_or(0.0),
                "potassium" => data.potassium.unwrap_or(0.0),
                "co2" => data.co2.unwrap_or(0.0),
                "pm25" => data.pm25.unwrap_or(0.0),
                "pm10" => data.pm10.unwrap_or(0.0),
                "wind_speed" => data.wind_speed.unwrap_or(0.0),
                "rainfall" => data.rainfall.unwrap_or(0.0),
                "solar_radiation" => data.solar_radiation.unwrap_or(0.0),
                _ => {
                    // Check if the feature is in additional_data
                    data.additional_data
                        .get(feature_name)
                        .copied()
                        .unwrap_or(0.0)
                }
            };
            
            features.push(value);
        }
        
        Ok(Array1::from(features))
    }
    
    /// Generate recommendations based on the detected disease
    fn generate_recommendations(&self, disease_type: &DiseaseType) -> Vec<String> {
        match disease_type {
            DiseaseType::Bacterial(bacterial) => match bacterial {
                _ => vec![
                    "Remove and destroy infected plant parts".to_string(),
                    "Apply copper-based bactericide".to_string(),
                    "Ensure good air circulation".to_string(),
                    "Avoid overhead irrigation".to_string(),
                ],
            },
            DiseaseType::Fungal(fungal) => match fungal {
                _ => vec![
                    "Remove and destroy infected plant parts".to_string(),
                    "Apply appropriate fungicide".to_string(),
                    "Ensure good air circulation".to_string(),
                    "Avoid overhead irrigation".to_string(),
                    "Rotate crops in the affected area".to_string(),
                ],
            },
            DiseaseType::Viral(_) => vec![
                "Remove and destroy infected plants".to_string(),
                "Control insect vectors".to_string(),
                "Use virus-free planting material".to_string(),
                "Disinfect tools and equipment".to_string(),
            ],
            DiseaseType::Pest(pest) => match pest {
                _ => vec![
                    "Apply appropriate insecticide".to_string(),
                    "Introduce beneficial insects".to_string(),
                    "Use sticky traps".to_string(),
                    "Ensure good plant health".to_string(),
                ],
            },
            DiseaseType::NutrientDeficiency(nutrient) => match nutrient {
                _ => vec![
                    "Apply appropriate fertilizer".to_string(),
                    "Adjust soil pH if necessary".to_string(),
                    "Ensure proper irrigation".to_string(),
                ],
            },
            DiseaseType::Other(_) => vec![
                "Consult with a plant pathologist".to_string(),
                "Monitor the affected plants closely".to_string(),
                "Isolate affected plants if possible".to_string(),
            ],
        }
    }
}

#[async_trait]
impl SensorDataModel<DiseaseDetection> for DiseaseSensorModel {
    async fn predict(&self, data: &SensorData) -> AgriResult<PredictionResult<DiseaseDetection>> {
        let model = self.model.as_ref().ok_or_else(|| {
            AgriMonitorError::ModelLoadError("Model not loaded".to_string())
        })?;
        
        // Extract features from sensor data
        let features = self.extract_features(data)?;
        
        // Make prediction
        let features_2d = features.insert_axis(Axis(0));
        let prediction = model.predict(&features_2d);
        let predicted_class = prediction[0];
        
        // Get disease type from predicted class
        let disease_type = self
            .disease_types
            .get(predicted_class)
            .cloned();
        
        // Calculate confidence (simplified)
        // In a real model, we would use probabilities from the model
        let confidence = 0.8; // Placeholder
        
        // Create disease detection result
        let disease_detected = confidence >= self.confidence_threshold;
        
        let disease_detection = DiseaseDetection {
            disease_detected,
            disease_type: if disease_detected { disease_type } else { None },
            severity: confidence,
            affected_area: None, // Not applicable for sensor data
            recommendations: if disease_detected && disease_type.is_some() {
                self.generate_recommendations(disease_type.as_ref().unwrap())
            } else {
                vec![]
            },
            additional_info: HashMap::new(),
        };
        
        // Create prediction result
        let result = PredictionResult {
            timestamp: data.timestamp,
            prediction: disease_detection,
            confidence,
            additional_info: HashMap::new(),
        };
        
        Ok(result)
    }
    
    async fn train(&mut self, data: &[SensorData], labels: &[DiseaseDetection]) -> AgriResult<()> {
        if data.is_empty() || labels.is_empty() || data.len() != labels.len() {
            return Err(AgriMonitorError::DataProcessingError(
                "Invalid training data".to_string(),
            ));
        }
        
        // Extract features from all sensor data
        let mut features = Vec::with_capacity(data.len());
        for sensor_data in data {
            features.push(self.extract_features(sensor_data)?);
        }
        
        // Convert features to 2D array
        let features_array = Array2::from_shape_vec(
            (features.len(), self.feature_names.len()),
            features.into_iter().flatten().collect(),
        )
        .map_err(|e| {
            AgriMonitorError::DataProcessingError(format!("Failed to create features array: {}", e))
        })?;
        
        // Extract target classes from labels
        let mut targets = Vec::with_capacity(labels.len());
        for label in labels {
            if let Some(disease_type) = &label.disease_type {
                // Find the index of the disease type in the list
                let class = self
                    .disease_types
                    .iter()
                    .position(|d| d == disease_type)
                    .unwrap_or_else(|| {
                        // If not found, add it to the list
                        self.disease_types.push(disease_type.clone());
                        self.disease_types.len() - 1
                    });
                
                targets.push(class);
            } else {
                // If no disease, use a special class (e.g., 0)
                targets.push(0);
            }
        }
        
        let targets_array = Array1::from(targets);
        
        // Create dataset
        let dataset = Dataset::new(features_array, targets_array)
            .map_err(|e| AgriMonitorError::DataProcessingError(format!("Failed to create dataset: {}", e)))?;
        
        // Train the model
        let model = DecisionTree::params()
            .split_quality(SplitQuality::Gini)
            .max_depth(10)
            .min_samples_split(5)
            .fit(&dataset)
            .map_err(|e| AgriMonitorError::ModelLoadError(format!("Failed to train model: {}", e)))?;
        
        self.model = Some(model);
        
        info!("Trained disease detection model with {} samples", data.len());
        
        Ok(())
    }
    
    async fn save<P: AsRef<Path> + Send + Sync>(&self, path: P) -> AgriResult<()> {
        let model = self.model.as_ref().ok_or_else(|| {
            AgriMonitorError::ModelLoadError("No model to save".to_string())
        })?;
        
        // Serialize the model
        let model_bytes = bincode::serialize(model)
            .map_err(|e| AgriMonitorError::ModelLoadError(format!("Failed to serialize model: {}", e)))?;
        
        // Save the model to a file
        tokio::fs::write(&path, model_bytes)
            .await
            .map_err(|e| AgriMonitorError::ModelLoadError(format!("Failed to save model: {}", e)))?;
        
        info!("Saved disease detection model to {:?}", path.as_ref());
        
        Ok(())
    }
    
    async fn load<P: AsRef<Path> + Send + Sync>(&mut self, path: P) -> AgriResult<()> {
        // Read the model from a file
        let model_bytes = tokio::fs::read(&path)
            .await
            .map_err(|e| AgriMonitorError::ModelLoadError(format!("Failed to read model file: {}", e)))?;
        
        // Deserialize the model
        let model: DecisionTree<f32, usize> = bincode::deserialize(&model_bytes)
            .map_err(|e| AgriMonitorError::ModelLoadError(format!("Failed to deserialize model: {}", e)))?;
        
        self.model = Some(model);
        self.model_path = Some(path.as_ref().to_path_buf());
        
        info!("Loaded disease detection model from {:?}", path.as_ref());
        
        Ok(())
    }
}

