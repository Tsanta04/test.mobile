//! Tensor utilities for machine learning models
//!
//! This module provides utility functions for working with tensors and arrays.

use crate::error::{AgriMonitorError, AgriResult};
use ndarray::{Array1, Array2, ArrayView1};
use rand::seq::SliceRandom;
use std::ops::Range;

/// Normalize an array to the range [0, 1]
pub fn normalize(array: &Array1<f32>) -> AgriResult<Array1<f32>> {
    if array.is_empty() {
        return Err(AgriMonitorError::DataProcessingError(
            "Cannot normalize empty array".to_string(),
        ));
    }
    
    let min = array.fold(f32::INFINITY, |a, &b| a.min(b));
    let max = array.fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    
    if (max - min).abs() < f32::EPSILON {
        // If all values are the same, return an array of 0.5
        return Ok(Array1::from_elem(array.len(), 0.5));
    }
    
    let normalized = array.mapv(|x| (x - min) / (max - min));
    Ok(normalized)
}

/// Normalize an array to a specific range
pub fn normalize_to_range(array: &Array1<f32>, range: Range<f32>) -> AgriResult<Array1<f32>> {
    let normalized = normalize(array)?;
    let range_size = range.end - range.start;
    let scaled = normalized.mapv(|x| x * range_size + range.start);
    Ok(scaled)
}

/// Standardize an array (z-score normalization)
pub fn standardize(array: &Array1<f32>) -> AgriResult<Array1<f32>> {
    if array.is_empty() {
        return Err(AgriMonitorError::DataProcessingError(
            "Cannot standardize empty array".to_string(),
        ));
    }
    
    let mean = array.mean().unwrap_or(0.0);
    let std_dev = standard_deviation(array.view())?;
    
    if std_dev < f32::EPSILON {
        // If standard deviation is close to zero, return zeros
        return Ok(Array1::zeros(array.len()));
    }
    
    let standardized = array.mapv(|x| (x - mean) / std_dev);
    Ok(standardized)
}

/// Calculate the standard deviation of an array
pub fn standard_deviation(array: ArrayView1<f32>) -> AgriResult<f32> {
    if array.is_empty() {
        return Err(AgriMonitorError::DataProcessingError(
            "Cannot calculate standard deviation of empty array".to_string(),
        ));
    }
    
    let mean = array.mean().unwrap_or(0.0);
    let variance = array.mapv(|x| (x - mean).powi(2)).mean().unwrap_or(0.0);
    let std_dev = variance.sqrt();
    
    Ok(std_dev)
}

/// One-hot encode a categorical variable
pub fn one_hot_encode(values: &[usize], num_categories: usize) -> AgriResult<Array2<f32>> {
    let mut encoded = Array2::zeros((values.len(), num_categories));
    
    for (i, &value) in values.iter().enumerate() {
        if value >= num_categories {
            return Err(AgriMonitorError::DataProcessingError(
                format!("Value {} is out of range for one-hot encoding with {} categories", value, num_categories),
            ));
        }
        
        encoded[[i, value]] = 1.0;
    }
    
    Ok(encoded)
}

/// Split an array into training and testing sets
pub fn train_test_split<T: Clone>(
    data: &[T],
    test_ratio: f32,
) -> AgriResult<(Vec<T>, Vec<T>)> {
    if data.is_empty() {
        return Err(AgriMonitorError::DataProcessingError(
            "Cannot split empty data".to_string(),
        ));
    }
    
    if test_ratio <= 0.0 || test_ratio >= 1.0 {
        return Err(AgriMonitorError::DataProcessingError(
            "Test ratio must be between 0 and 1".to_string(),
        ));
    }
    
    let test_size = (data.len() as f32 * test_ratio).round() as usize;
    let train_size = data.len() - test_size;
    
    // Create a shuffled copy of the data
    let mut shuffled = data.to_vec();
    let mut rng = rand::thread_rng();
    shuffled.shuffle(&mut rng);
    
    let train = shuffled[..train_size].to_vec();
    let test = shuffled[train_size..].to_vec();
    
    Ok((train, test))
}
