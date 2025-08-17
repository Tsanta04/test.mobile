//! Types de données pour le module de prédiction du taux d'eau
//!
//! Ce module définit les types de données spécifiques au module de taux d'eau,
//! notamment les prédictions de taux d'humidité et les recommandations d'arrosage.

use chrono::{DateTime, Utc};
use common::data::{GeoLocation, ImageData, SensorData};
use serde::{Deserialize, Serialize};

/// Statut d'humidité du sol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SoilMoistureStatus {
    /// Sol très sec, irrigation urgente nécessaire
    Dry,
    /// Sol légèrement sec, irrigation recommandée
    SlightlyDry,
    /// Niveau d'humidité optimal
    Optimal,
    /// Sol humide, pas d'irrigation nécessaire
    Wet,
    /// Sol saturé, risque d'engorgement
    Saturated,
}

impl SoilMoistureStatus {
    /// Convertit un pourcentage d'humidité en statut
    pub fn from_percentage(percentage: f32) -> Self {
        match percentage {
            p if p < 0.2 => SoilMoistureStatus::Dry,
            p if p < 0.4 => SoilMoistureStatus::SlightlyDry,
            p if p < 0.7 => SoilMoistureStatus::Optimal,
            p if p < 0.9 => SoilMoistureStatus::Wet,
            _ => SoilMoistureStatus::Saturated,
        }
    }

    /// Convertit le statut en chaîne de caractères
    pub fn to_string(&self) -> String {
        match self {
            SoilMoistureStatus::Dry => "dry".to_string(),
            SoilMoistureStatus::SlightlyDry => "slightly_dry".to_string(),
            SoilMoistureStatus::Optimal => "optimal".to_string(),
            SoilMoistureStatus::Wet => "wet".to_string(),
            SoilMoistureStatus::Saturated => "saturated".to_string(),
        }
    }
}

/// Niveau d'urgence pour l'irrigation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IrrigationUrgency {
    /// Pas d'irrigation nécessaire
    None,
    /// Irrigation à prévoir dans les prochains jours
    Low,
    /// Irrigation recommandée prochainement
    Medium,
    /// Irrigation urgente nécessaire
    High,
}

impl IrrigationUrgency {
    /// Convertit le niveau d'urgence en chaîne de caractères
    pub fn to_string(&self) -> String {
        match self {
            IrrigationUrgency::None => "none".to_string(),
            IrrigationUrgency::Low => "low".to_string(),
            IrrigationUrgency::Medium => "medium".to_string(),
            IrrigationUrgency::High => "high".to_string(),
        }
    }
}

/// Prédiction du taux d'humidité du sol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterLevelPrediction {
    /// Horodatage de la prédiction
    pub timestamp: DateTime<Utc>,
    /// Taux d'humidité prédit (0.0 à 1.0)
    pub water_level: f32,
    /// Statut d'humidité du sol
    pub status: SoilMoistureStatus,
    /// Niveau de confiance de la prédiction (0.0 à 1.0)
    pub confidence: f32,
}

/// Recommandation d'arrosage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrrigationRecommendation {
    /// Indique si l'irrigation est nécessaire
    pub irrigation_needed: bool,
    /// Quantité d'eau recommandée en mm
    pub water_amount_mm: f32,
    /// Niveau d'urgence de l'irrigation
    pub urgency: IrrigationUrgency,
    /// Meilleur moment pour irriguer (ex: "matin", "soir")
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

/// Carte de taux d'humidité
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterLevelMap {
    /// Horodatage de la carte
    pub timestamp: DateTime<Utc>,
    /// Matrice 2D des taux d'humidité
    pub water_level_matrix: Vec<Vec<f32>>,
    /// Taux d'humidité moyen
    pub average_water_level: f32,
    /// Résolution de la carte
    pub resolution: MapResolution,
    /// Référence géographique
    pub geo_reference: GeoReference,
    /// Statut d'humidité global
    pub status: SoilMoistureStatus,
}

/// Données météorologiques
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    /// Température actuelle en °C
    pub temperature: f32,
    /// Humidité de l'air en %
    pub air_humidity: f32,
    /// Précipitations prévues dans les prochaines 24h en mm
    pub forecast_precipitation_24h: f32,
    /// Vitesse du vent en m/s
    pub wind_speed: Option<f32>,
    /// Évapotranspiration potentielle en mm/jour
    pub evapotranspiration: Option<f32>,
}

/// Données de culture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CropData {
    /// Type de culture
    pub crop_type: String,
    /// Stade de croissance
    pub growth_stage: String,
    /// Coefficient cultural (Kc)
    pub crop_coefficient: f32,
    /// Profondeur des racines en cm
    pub root_depth_cm: f32,
    /// Seuil d'humidité critique en dessous duquel la plante souffre
    pub critical_moisture_threshold: f32,
}

/// Requête de prédiction basée sur les données de capteurs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorPredictionRequest {
    /// Données des capteurs
    pub sensor_data: SensorData,
    /// Données météorologiques
    pub weather_data: WeatherData,
    /// Données de culture
    pub crop_data: CropData,
    /// Identifiant du champ
    pub field_id: Option<String>,
}

/// Requête de prédiction basée sur une image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePredictionRequest {
    /// Données de l'image
    pub image_data: ImageData,
    /// Données météorologiques
    pub weather_data: WeatherData,
    /// Données de culture
    pub crop_data: CropData,
    /// Identifiant du champ
    pub field_id: Option<String>,
}

