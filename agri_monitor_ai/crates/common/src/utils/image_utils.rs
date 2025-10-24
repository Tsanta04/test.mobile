//! Image processing utilities
//!
//! This module provides utility functions for processing images.

use crate::data::ImageData;
use crate::error::{AgriMonitorError, AgriResult};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use ndarray::{Array, Array3};
use std::path::Path;

/// Load an image from a file
pub fn load_image<P: AsRef<Path>>(path: P) -> AgriResult<DynamicImage> {
    image::open(path).map_err(|e| {
        AgriMonitorError::ImageProcessingError(format!("Failed to load image: {}", e))
    })
}

/// Convert an image to RGB format
pub fn to_rgb(image: &DynamicImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    image.to_rgb8()
}

/// Resize an image to the specified dimensions
pub fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
}

/// Convert an image to a 3D ndarray (height, width, channels)
pub fn image_to_array(image: &DynamicImage) -> Array3<f32> {
    let rgb = to_rgb(image);
    let (width, height) = rgb.dimensions();
    let mut array = Array::zeros((height as usize, width as usize, 3));
    
    for y in 0..height {
        for x in 0..width {
            let pixel = rgb.get_pixel(x, y);
            array[[y as usize, x as usize, 0]] = pixel[0] as f32 / 255.0;
            array[[y as usize, x as usize, 1]] = pixel[1] as f32 / 255.0;
            array[[y as usize, x as usize, 2]] = pixel[2] as f32 / 255.0;
        }
    }
    
    array
}

/// Calculate NDVI (Normalized Difference Vegetation Index) from multispectral image
/// 
/// NDVI = (NIR - Red) / (NIR + Red)
/// 
/// This is a simplified version that assumes the image has NIR and Red channels.
pub fn calculate_ndvi(nir_channel: &[f32], red_channel: &[f32]) -> AgriResult<Vec<f32>> {
    if nir_channel.len() != red_channel.len() {
        return Err(AgriMonitorError::ImageProcessingError(
            "NIR and Red channels must have the same length".to_string(),
        ));
    }
    
    let mut ndvi = Vec::with_capacity(nir_channel.len());
    
    for (nir, red) in nir_channel.iter().zip(red_channel.iter()) {
        let denominator = nir + red;
        
        let value = if denominator.abs() < f32::EPSILON {
            0.0 // Avoid division by zero
        } else {
            (nir - red) / denominator
        };
        
        ndvi.push(value);
    }
    
    Ok(ndvi)
}

/// Extract metadata from an image
pub fn extract_image_metadata(image_data: &ImageData) -> AgriResult<std::collections::HashMap<String, String>> {
    // Créer une image à partir des données binaires
    let image = image::load_from_memory(&image_data.image)
        .map_err(|e| AgriMonitorError::ImageProcessingError(format!("Failed to load image from memory: {}", e)))?;
    
    let (width, height) = image.dimensions();
    
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("width".to_string(), width.to_string());
    metadata.insert("height".to_string(), height.to_string());
    metadata.insert("format".to_string(), format!("{:?}", image.color()));
    metadata.insert("image_type".to_string(), image_data.image_type.clone());
    metadata.insert("timestamp".to_string(), image_data.timestamp.to_rfc3339());
    
    Ok(metadata)
}
