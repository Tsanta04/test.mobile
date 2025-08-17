//! Module de prédiction de la fertilité du sol et de recommandation de fertilisation
//!
//! Ce module fournit des fonctionnalités pour:
//! - Prédire la fertilité du sol à partir de données de capteurs
//! - Générer des cartes de fertilité à partir d'images
//! - Fournir des recommandations de fertilisation
//! - Analyser les niveaux de nutriments

pub mod models;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

use crate::models::{
    image_model::FertilityImageModel,
    sensor_model::FertilitySensorModel,
};
use crate::types::{
    FertilityMap, FertilityPrediction, FertilizationRecommendation,
    SensorPredictionRequest, ImagePredictionRequest
};
use common::error::AgriResult;
use common::models::Model;

/// Gestionnaire principal pour les prédictions de fertilité
pub struct FertilityPredictor {
    sensor_model: FertilitySensorModel,
    image_model: FertilityImageModel,
}

impl FertilityPredictor {
    /// Crée une nouvelle instance du prédicteur de fertilité
    pub fn new() -> Self {
        Self {
            sensor_model: FertilitySensorModel::new(),
            image_model: FertilityImageModel::new(),
        }
    }

    /// Charge les modèles à partir des chemins spécifiés
    pub async fn load_models(
        &mut self,
        sensor_model_path: &str,
        image_model_path: &str,
    ) -> AgriResult<()> {
        self.sensor_model.load(sensor_model_path).await?;
        self.image_model.load(image_model_path).await?;
        Ok(())
    }

    /// Prédit la fertilité à partir de données de capteurs
    pub async fn predict_from_sensor_data(
        &self,
        request: SensorPredictionRequest,
    ) -> AgriResult<(FertilityPrediction, FertilizationRecommendation)> {
        // Prédire la fertilité
        let prediction = self.sensor_model.predict(&request.sensor_data).await?;
        
        // Générer une recommandation de fertilisation basée sur la prédiction
        let recommendation = utils::calculate_fertilization_recommendation(
            &prediction, 
            &request.soil_data, 
            &request.crop_data
        )?;
        
        Ok((prediction, recommendation))
    }

    /// Prédit la fertilité à partir d'une image
    pub async fn predict_from_image(
        &self,
        request: ImagePredictionRequest,
    ) -> AgriResult<(FertilityMap, FertilizationRecommendation)> {
        // Générer une carte de fertilité à partir de l'image
        let fertility_map = self.image_model.predict(&request.image_data).await?;
        
        // Générer une recommandation de fertilisation basée sur la carte de fertilité
        let recommendation = self.generate_fertilization_recommendation_from_map(
            &fertility_map, 
            &request
        )?;
        
        Ok((fertility_map, recommendation))
    }

    /// Génère une recommandation de fertilisation basée sur une carte de fertilité
    fn generate_fertilization_recommendation_from_map(
        &self,
        fertility_map: &FertilityMap,
        request: &ImagePredictionRequest,
    ) -> AgriResult<FertilizationRecommendation> {
        // Créer une prédiction de fertilité à partir de la carte
        let prediction = FertilityPrediction {
            timestamp: fertility_map.timestamp,
            fertility_score: fertility_map.average_fertility,
            status: fertility_map.status.clone(),
            ph_level: 6.5, // Valeur par défaut, à remplacer par une valeur réelle si disponible
            nutrient_levels: self.extract_nutrient_levels_from_map(fertility_map)?,
            confidence: 0.8,
        };

        // Utiliser les utilitaires pour calculer la recommandation
        utils::calculate_fertilization_recommendation(
            &prediction, 
            &request.soil_data, 
            &request.crop_data
        )
    }

    /// Extrait les niveaux de nutriments à partir d'une carte de fertilité
    fn extract_nutrient_levels_from_map(
        &self,
        fertility_map: &FertilityMap,
    ) -> AgriResult<types::NutrientLevels> {
        // Calculer les moyennes des matrices de nutriments
        let mut nitrogen = 0.0;
        let mut phosphorus = 0.0;
        let mut potassium = 0.0;
        let mut count = 0;

        if let Some(n_matrix) = fertility_map.nutrient_matrices.get("nitrogen") {
            for row in n_matrix {
                for &value in row {
                    nitrogen += value;
                    count += 1;
                }
            }
            nitrogen /= count as f32;
        }

        count = 0;
        if let Some(p_matrix) = fertility_map.nutrient_matrices.get("phosphorus") {
            for row in p_matrix {
                for &value in row {
                    phosphorus += value;
                    count += 1;
                }
            }
            phosphorus /= count as f32;
        }

        count = 0;
        if let Some(k_matrix) = fertility_map.nutrient_matrices.get("potassium") {
            for row in k_matrix {
                for &value in row {
                    potassium += value;
                    count += 1;
                }
            }
            potassium /= count as f32;
        }

        // Créer l'objet NutrientLevels
        let nutrient_levels = types::NutrientLevels {
            nitrogen,
            phosphorus,
            potassium,
            organic_matter: Some(3.0), // Valeur par défaut
            calcium: None,
            magnesium: None,
            sulfur: None,
            zinc: None,
            iron: None,
            manganese: None,
            copper: None,
            boron: None,
            others: std::collections::HashMap::new(),
        };

        Ok(nutrient_levels)
    }
}

impl Default for FertilityPredictor {
    fn default() -> Self {
        Self::new()
    }
}
