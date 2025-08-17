//! Types de données pour le module de prédiction de la fertilité du sol
//!
//! Ce module définit les types de données spécifiques au module de fertilité,
//! notamment les prédictions de fertilité et les recommandations de fertilisation.

use chrono::{DateTime, Utc};
use common::data::{GeoLocation, ImageData, SensorData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Statut de fertilité du sol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SoilFertilityStatus {
    /// Sol très pauvre en nutriments
    VeryPoor,
    /// Sol pauvre en nutriments
    Poor,
    /// Niveau de fertilité moyen
    Moderate,
    /// Sol fertile
    Good,
    /// Sol très fertile
    Excellent,
}

impl SoilFertilityStatus {
    /// Convertit un score de fertilité en statut
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s < 0.2 => SoilFertilityStatus::VeryPoor,
            s if s < 0.4 => SoilFertilityStatus::Poor,
            s if s < 0.6 => SoilFertilityStatus::Moderate,
            s if s < 0.8 => SoilFertilityStatus::Good,
            _ => SoilFertilityStatus::Excellent,
        }
    }

    /// Convertit le statut en chaîne de caractères
    pub fn to_string(&self) -> String {
        match self {
            SoilFertilityStatus::VeryPoor => "very_poor".to_string(),
            SoilFertilityStatus::Poor => "poor".to_string(),
            SoilFertilityStatus::Moderate => "moderate".to_string(),
            SoilFertilityStatus::Good => "good".to_string(),
            SoilFertilityStatus::Excellent => "excellent".to_string(),
        }
    }
}

/// Niveau de priorité pour la fertilisation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FertilizationPriority {
    /// Pas de fertilisation nécessaire
    None,
    /// Fertilisation à prévoir dans les prochains mois
    Low,
    /// Fertilisation recommandée prochainement
    Medium,
    /// Fertilisation urgente nécessaire
    High,
}

impl FertilizationPriority {
    /// Convertit le niveau de priorité en chaîne de caractères
    pub fn to_string(&self) -> String {
        match self {
            FertilizationPriority::None => "none".to_string(),
            FertilizationPriority::Low => "low".to_string(),
            FertilizationPriority::Medium => "medium".to_string(),
            FertilizationPriority::High => "high".to_string(),
        }
    }
}

/// Niveaux de nutriments du sol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientLevels {
    /// Niveau d'azote (N) en ppm
    pub nitrogen: f32,
    /// Niveau de phosphore (P) en ppm
    pub phosphorus: f32,
    /// Niveau de potassium (K) en ppm
    pub potassium: f32,
    /// Niveau de matière organique en %
    pub organic_matter: Option<f32>,
    /// Niveau de calcium en ppm
    pub calcium: Option<f32>,
    /// Niveau de magnésium en ppm
    pub magnesium: Option<f32>,
    /// Niveau de soufre en ppm
    pub sulfur: Option<f32>,
    /// Niveau de zinc en ppm
    pub zinc: Option<f32>,
    /// Niveau de fer en ppm
    pub iron: Option<f32>,
    /// Niveau de manganèse en ppm
    pub manganese: Option<f32>,
    /// Niveau de cuivre en ppm
    pub copper: Option<f32>,
    /// Niveau de bore en ppm
    pub boron: Option<f32>,
    /// Autres nutriments
    pub others: HashMap<String, f32>,
}

/// Prédiction de la fertilité du sol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilityPrediction {
    /// Horodatage de la prédiction
    pub timestamp: DateTime<Utc>,
    /// Score global de fertilité (0.0 à 1.0)
    pub fertility_score: f32,
    /// Statut de fertilité du sol
    pub status: SoilFertilityStatus,
    /// Niveau de pH du sol
    pub ph_level: f32,
    /// Niveaux de nutriments
    pub nutrient_levels: NutrientLevels,
    /// Niveau de confiance de la prédiction (0.0 à 1.0)
    pub confidence: f32,
}

/// Recommandation de fertilisation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilizationRecommendation {
    /// Indique si la fertilisation est nécessaire
    pub fertilization_needed: bool,
    /// Priorité de la fertilisation
    pub priority: FertilizationPriority,
    /// Recommandations d'engrais par nutriment (en kg/ha)
    pub fertilizer_amounts: HashMap<String, f32>,
    /// Type d'engrais recommandé
    pub recommended_fertilizer_type: Option<String>,
    /// Meilleure période pour fertiliser
    pub optimal_time: Option<String>,
    /// Recommandations supplémentaires
    pub notes: Vec<String>,
}

/// Résolution d'une carte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapResolution {
    /// Largeur de la carte en pixels
    pub width: usize,
    /// Hauteur de la carte en pixels
    pub height: usize,
    /// Mètres par pixel
    pub meters_per_pixel: f32,
}

/// Référence géographique d'une carte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoReference {
    /// Coordonnées du coin supérieur gauche
    pub top_left: GeoLocation,
    /// Coordonnées du coin inférieur droit
    pub bottom_right: GeoLocation,
}

/// Carte de fertilité du sol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilityMap {
    /// Horodatage de la carte
    pub timestamp: DateTime<Utc>,
    /// Matrice 2D des scores de fertilité
    pub fertility_matrix: Vec<Vec<f32>>,
    /// Matrices des niveaux de nutriments (N, P, K)
    pub nutrient_matrices: HashMap<String, Vec<Vec<f32>>>,
    /// Score de fertilité moyen
    pub average_fertility: f32,
    /// Résolution de la carte
    pub resolution: MapResolution,
    /// Référence géographique
    pub geo_reference: GeoReference,
    /// Statut de fertilité global
    pub status: SoilFertilityStatus,
}

/// Données de sol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoilData {
    /// Type de sol
    pub soil_type: String,
    /// Texture du sol
    pub soil_texture: String,
    /// Capacité d'échange cationique (CEC)
    pub cec: Option<f32>,
    /// Densité apparente du sol (g/cm³)
    pub bulk_density: Option<f32>,
    /// Profondeur du sol en cm
    pub soil_depth_cm: f32,
}

/// Données de culture pour la fertilité
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropFertilityData {
    /// Type de culture
    pub crop_type: String,
    /// Stade de croissance
    pub growth_stage: String,
    /// Besoins en azote (N) en kg/ha
    pub nitrogen_requirement: f32,
    /// Besoins en phosphore (P) en kg/ha
    pub phosphorus_requirement: f32,
    /// Besoins en potassium (K) en kg/ha
    pub potassium_requirement: f32,
    /// Plage de pH optimale
    pub optimal_ph_range: (f32, f32),
    /// Besoins en autres nutriments
    pub other_requirements: HashMap<String, f32>,
}

/// Requête de prédiction basée sur les données de capteurs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorPredictionRequest {
    /// Données des capteurs
    pub sensor_data: SensorData,
    /// Données de sol
    pub soil_data: SoilData,
    /// Données de culture
    pub crop_data: CropFertilityData,
    /// Identifiant du champ
    pub field_id: Option<String>,
    /// Historique des fertilisations
    pub fertilization_history: Option<Vec<FertilizationEvent>>,
}

/// Requête de prédiction basée sur une image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePredictionRequest {
    /// Données de l'image
    pub image_data: ImageData,
    /// Données de sol
    pub soil_data: SoilData,
    /// Données de culture
    pub crop_data: CropFertilityData,
    /// Identifiant du champ
    pub field_id: Option<String>,
}

/// Événement de fertilisation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FertilizationEvent {
    /// Date de l'événement
    pub date: DateTime<Utc>,
    /// Type d'engrais utilisé
    pub fertilizer_type: String,
    /// Quantité d'engrais en kg/ha
    pub amount_kg_per_ha: f32,
    /// Nutriments apportés
    pub nutrients_applied: HashMap<String, f32>,
}

