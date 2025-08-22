//! Module de prédiction du taux d'eau et de recommandation d'arrosage
//!
//! Ce module fournit des fonctionnalités pour:
//! - Prédire le taux d'humidité du sol à partir de données de capteurs
//! - Générer des cartes d'humidité à partir d'images
//! - Fournir des recommandations d'arrosage (décision et quantité)
//! - Calculer le déficit hydrique

pub mod models;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

use crate::models::{
    image_model::WaterLevelImageModel,
    sensor_model::WaterLevelSensorModel,
};
use crate::types::{
    IrrigationRecommendation, WaterLevelMap, WaterLevelPrediction,
    SensorPredictionRequest, ImagePredictionRequest
};
use common::error::AgriResult;
use common::models::{Model, SensorDataModel, ImageModel};

/// Gestionnaire principal pour les prédictions de taux d'eau
pub struct WaterLevelPredictor {
    sensor_model: WaterLevelSensorModel,
    image_model: WaterLevelImageModel,
}

impl WaterLevelPredictor {
    /// Crée une nouvelle instance du prédicteur de taux d'eau
    pub fn new() -> Self {
        Self {
            sensor_model: WaterLevelSensorModel::new(),
            image_model: WaterLevelImageModel::new(),
        }
    }

    /// Charge les modèles à partir des chemins spécifiés
    pub async fn load_models(
        &mut self,
        sensor_model_path: &str,
        image_model_path: &str,
    ) -> AgriResult<()> {
        Model::load(&mut self.sensor_model, sensor_model_path).await?;
        Model::load(&mut self.image_model, image_model_path).await?;
        Ok(())
    }

    /// Prédit le taux d'humidité à partir de données de capteurs
    pub async fn predict_from_sensor_data(
        &self,
        request: SensorPredictionRequest,
    ) -> AgriResult<(WaterLevelPrediction, IrrigationRecommendation)> {
        // Prédire le taux d'humidité
        let prediction_result = SensorDataModel::predict(&self.sensor_model, &request.sensor_data).await?;
        let prediction = prediction_result.prediction;
        
        // Générer une recommandation d'arrosage basée sur la prédiction
        let recommendation = self.generate_irrigation_recommendation(&prediction, &request).await?;
        
        Ok((prediction, recommendation))
    }

    /// Prédit le taux d'humidité à partir d'une image
    pub async fn predict_from_image(
        &self,
        request: ImagePredictionRequest,
    ) -> AgriResult<(WaterLevelMap, IrrigationRecommendation)> {
        // Générer une carte d'humidité à partir de l'image
        let water_map_result = ImageModel::predict(&self.image_model, &request.image_data).await?;
        let water_map = water_map_result.prediction;
        
        // Générer une recommandation d'arrosage basée sur la carte d'humidité
        let recommendation = self.generate_irrigation_recommendation_from_map(&water_map, &request).await?;
        
        Ok((water_map, recommendation))
    }

    /// Génère une recommandation d'arrosage basée sur une prédiction de taux d'humidité
    async fn generate_irrigation_recommendation(
        &self,
        prediction: &WaterLevelPrediction,
        request: &SensorPredictionRequest,
    ) -> AgriResult<IrrigationRecommendation> {
        // Utiliser les utilitaires pour calculer la recommandation
        utils::calculate_irrigation_recommendation(prediction, &request.crop_data, &request.weather_data)
    }

    /// Génère une recommandation d'arrosage basée sur une carte d'humidité
    async fn generate_irrigation_recommendation_from_map(
        &self,
        water_map: &WaterLevelMap,
        request: &ImagePredictionRequest,
    ) -> AgriResult<IrrigationRecommendation> {
        // Utiliser les utilitaires pour calculer la recommandation
        utils::calculate_irrigation_recommendation_from_map(water_map, &request.crop_data, &request.weather_data)
    }
}

impl Default for WaterLevelPredictor {
    fn default() -> Self {
        Self::new()
    }
}
