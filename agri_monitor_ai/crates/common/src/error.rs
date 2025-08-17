//! Error types for the agricultural monitoring system
//!
//! This module defines common error types used across all prediction modules.

use thiserror::Error;

/// Common error type for all prediction modules
#[derive(Error, Debug)]
pub enum AgriMonitorError {
    /// Error when loading model
    #[error("Failed to load model: {0}")]
    ModelLoadError(String),

    /// Error when model is not loaded
    #[error("Model not loaded: {0}")]
    ModelNotLoaded(String),

    /// Error when training model
    #[error("Failed to train model: {0}")]
    ModelTrainingError(String),

    /// Error when making predictions
    #[error("Failed to make prediction: {0}")]
    PredictionError(String),

    /// Error when processing input data
    #[error("Failed to process input data: {0}")]
    DataProcessingError(String),

    /// Error when connecting to database
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Error when processing images
    #[error("Image processing error: {0}")]
    ImageProcessingError(String),

    /// Error when fetching external data
    #[error("External data fetch error: {0}")]
    ExternalDataError(String),

    /// Error when validating input
    #[error("Input validation error: {0}")]
    ValidationError(String),

    /// Error when input is invalid
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Generic error
    #[error("Error: {0}")]
    GenericError(String),
}

/// Result type alias for AgriMonitorError
pub type AgriResult<T> = Result<T, AgriMonitorError>;

