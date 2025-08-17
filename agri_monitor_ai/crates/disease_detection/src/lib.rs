//! Disease detection module for agricultural monitoring
//!
//! This module provides functionality to detect plant diseases using:
//! 1. Raw sensor data (soil moisture, temperature, etc.)
//! 2. Image data (RGB or multispectral images of plants)
//!
//! The module implements two types of models:
//! - Random Forest for sensor data
//! - CNN (Convolutional Neural Network) for image data

pub mod models;
pub mod types;
pub mod utils;

