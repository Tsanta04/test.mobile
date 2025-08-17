//! Common model interfaces and traits
//!
//! This module defines common traits and interfaces for machine learning models
//! used across all prediction modules.

use crate::data::{ImageData, PredictionResult, SensorData};
use crate::error::AgriResult;
use async_trait::async_trait;
use std::path::Path;

/// Trait for models that make predictions from raw sensor data
#[async_trait]
pub trait SensorDataModel<T> {
    /// Make a prediction from sensor data
    async fn predict(&self, data: &SensorData) -> AgriResult<PredictionResult<T>>;
    
    /// Train the model with new data
    async fn train(&mut self, data: &[SensorData], labels: &[T]) -> AgriResult<()>;
    
    /// Save the model to a file
    async fn save<P: AsRef<Path> + Send + Sync>(&self, path: P) -> AgriResult<()>;
    
    /// Load the model from a file
    async fn load<P: AsRef<Path> + Send + Sync>(&mut self, path: P) -> AgriResult<()>;
}

/// Trait for models that make predictions from images
#[async_trait]
pub trait ImageModel<T> {
    /// Make a prediction from an image
    async fn predict(&self, data: &ImageData) -> AgriResult<PredictionResult<T>>;
    
    /// Train the model with new data
    async fn train(&mut self, data: &[ImageData], labels: &[T]) -> AgriResult<()>;
    
    /// Save the model to a file
    async fn save<P: AsRef<Path> + Send + Sync>(&self, path: P) -> AgriResult<()>;
    
    /// Load the model from a file
    async fn load<P: AsRef<Path> + Send + Sync>(&mut self, path: P) -> AgriResult<()>;
}

/// Trait for models that can be updated incrementally
#[async_trait]
pub trait IncrementalModel {
    /// Update the model with new data
    async fn update(&mut self, data: &[SensorData]) -> AgriResult<()>;
}

/// Trait for models that can explain their predictions
#[async_trait]
pub trait ExplainableModel<T, E> {
    /// Explain a prediction
    async fn explain(&self, prediction: &PredictionResult<T>) -> AgriResult<E>;
}

