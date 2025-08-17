//! Types for disease detection
//!
//! This module defines types specific to disease detection.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Disease detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseDetection {
    /// Whether a disease was detected
    pub disease_detected: bool,
    
    /// The type of disease detected
    pub disease_type: Option<DiseaseType>,
    
    /// The severity of the disease (0.0-1.0)
    pub severity: f32,
    
    /// The affected area (percentage of plant affected)
    pub affected_area: Option<f32>,
    
    /// Recommended actions to take
    pub recommendations: Vec<String>,
    
    /// Additional information about the disease
    pub additional_info: HashMap<String, String>,
}

/// Disease type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiseaseType {
    /// Bacterial diseases
    Bacterial(BacterialDisease),
    
    /// Fungal diseases
    Fungal(FungalDisease),
    
    /// Viral diseases
    Viral(ViralDisease),
    
    /// Pest infestations
    Pest(PestType),
    
    /// Nutrient deficiencies
    NutrientDeficiency(NutrientType),
    
    /// Other diseases
    Other(String),
}

/// Bacterial disease types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BacterialDisease {
    /// Bacterial blight
    BacterialBlight,
    
    /// Bacterial spot
    BacterialSpot,
    
    /// Bacterial wilt
    BacterialWilt,
    
    /// Crown gall
    CrownGall,
    
    /// Soft rot
    SoftRot,
    
    /// Other bacterial disease
    Other(String),
}

/// Fungal disease types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FungalDisease {
    /// Anthracnose
    Anthracnose,
    
    /// Black spot
    BlackSpot,
    
    /// Downy mildew
    DownyMildew,
    
    /// Powdery mildew
    PowderyMildew,
    
    /// Rust
    Rust,
    
    /// Leaf spot
    LeafSpot,
    
    /// Root rot
    RootRot,
    
    /// Other fungal disease
    Other(String),
}

/// Viral disease types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ViralDisease {
    /// Mosaic virus
    MosaicVirus,
    
    /// Leaf curl virus
    LeafCurlVirus,
    
    /// Yellowing virus
    YellowingVirus,
    
    /// Stunting virus
    StuntingVirus,
    
    /// Other viral disease
    Other(String),
}

/// Pest types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PestType {
    /// Aphids
    Aphids,
    
    /// Spider mites
    SpiderMites,
    
    /// Whiteflies
    Whiteflies,
    
    /// Thrips
    Thrips,
    
    /// Caterpillars
    Caterpillars,
    
    /// Beetles
    Beetles,
    
    /// Other pest
    Other(String),
}

/// Nutrient deficiency types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NutrientType {
    /// Nitrogen deficiency
    Nitrogen,
    
    /// Phosphorus deficiency
    Phosphorus,
    
    /// Potassium deficiency
    Potassium,
    
    /// Calcium deficiency
    Calcium,
    
    /// Magnesium deficiency
    Magnesium,
    
    /// Iron deficiency
    Iron,
    
    /// Zinc deficiency
    Zinc,
    
    /// Manganese deficiency
    Manganese,
    
    /// Boron deficiency
    Boron,
    
    /// Other nutrient deficiency
    Other(String),
}

/// Disease detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseDetectionConfig {
    /// Confidence threshold for disease detection (0.0-1.0)
    pub confidence_threshold: f32,
    
    /// Whether to generate recommendations
    pub generate_recommendations: bool,
    
    /// Whether to use sensor data for detection
    pub use_sensor_data: bool,
    
    /// Whether to use image data for detection
    pub use_image_data: bool,
    
    /// Path to the sensor data model
    pub sensor_model_path: Option<String>,
    
    /// Path to the image model
    pub image_model_path: Option<String>,
}

