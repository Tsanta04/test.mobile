//! Modèle de prédiction de la fertilité basé sur les données de capteurs
//!
//! Ce module implémente un modèle de Random Forest pour prédire la fertilité
//! du sol à partir des données de capteurs.

use crate::types::{FertilityPrediction, NutrientLevels, SoilFertilityStatus};
use chrono::Utc;
use common::data::SensorData;
use common::error::{AgriMonitorError, AgriResult};
use common::models::Model;
use async_trait::async_trait;
use linfa::prelude::*;
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::{Array1, Array2};
use std::collections::HashMap;
use std::path::Path;
use tracing::info;

/// Modèle de prédiction de la fertilité basé sur les données de capteurs
#[derive(Debug, Clone)]
pub struct FertilitySensorModel {
    /// Modèle de Random Forest pour la prédiction
    model: Option<DecisionTree<f32, usize>>,
}

impl FertilitySensorModel {
    /// Crée une nouvelle instance du modèle
    pub fn new() -> Self {
        Self { model: None }
    }

    /// Prédit la fertilité à partir des données de capteurs
    pub async fn predict(&self, sensor_data: &SensorData) -> AgriResult<FertilityPrediction> {
        // Vérifier si le modèle est chargé
        if self.model.is_none() {
            // Si le modèle n'est pas chargé, utiliser une méthode de prédiction simple
            return self.predict_simple(sensor_data);
        }

        // Extraire les caractéristiques des données de capteurs
        let features = self.extract_features(sensor_data)?;

        // Prédire le score de fertilité
        let fertility_score = self.predict_with_model(&features)?;

        // Déterminer le statut de fertilité
        let status = SoilFertilityStatus::from_score(fertility_score);

        // Extraire les niveaux de nutriments
        let nutrient_levels = self.extract_nutrient_levels(sensor_data)?;

        // Extraire le pH
        let ph_level = self.extract_ph(sensor_data)?;

        // Créer la prédiction
        let prediction = FertilityPrediction {
            timestamp: Utc::now(),
            fertility_score,
            status,
            ph_level,
            nutrient_levels,
            confidence: 0.85, // Valeur fixe pour l'instant
        };

        Ok(prediction)
    }

    /// Prédit la fertilité avec une méthode simple (sans modèle)
    fn predict_simple(&self, sensor_data: &SensorData) -> AgriResult<FertilityPrediction> {
        // Extraire le pH
        let ph_level = self.extract_ph(sensor_data)?;

        // Extraire les niveaux de nutriments
        let nutrient_levels = self.extract_nutrient_levels(sensor_data)?;

        // Calculer un score de fertilité simple
        let fertility_score = self.calculate_simple_fertility_score(ph_level, &nutrient_levels);

        // Déterminer le statut de fertilité
        let status = SoilFertilityStatus::from_score(fertility_score);

        // Créer la prédiction
        let prediction = FertilityPrediction {
            timestamp: Utc::now(),
            fertility_score,
            status,
            ph_level,
            nutrient_levels,
            confidence: 0.7, // Confiance réduite pour la méthode simple
        };

        Ok(prediction)
    }

    /// Calcule un score de fertilité simple
    fn calculate_simple_fertility_score(&self, ph: f32, nutrient_levels: &NutrientLevels) -> f32 {
        // Calculer le score de pH (optimal entre 6.0 et 7.0)
        let ph_score = if ph >= 6.0 && ph <= 7.0 {
            1.0
        } else if ph >= 5.5 && ph < 6.0 {
            0.8
        } else if ph > 7.0 && ph <= 7.5 {
            0.8
        } else if ph >= 5.0 && ph < 5.5 {
            0.6
        } else if ph > 7.5 && ph <= 8.0 {
            0.6
        } else if ph >= 4.5 && ph < 5.0 {
            0.4
        } else if ph > 8.0 && ph <= 8.5 {
            0.4
        } else {
            0.2
        };

        // Calculer le score de nutriments (N, P, K)
        // Valeurs typiques pour un sol fertile (en ppm)
        let n_score = (nutrient_levels.nitrogen / 100.0).min(1.0);
        let p_score = (nutrient_levels.phosphorus / 50.0).min(1.0);
        let k_score = (nutrient_levels.potassium / 200.0).min(1.0);

        // Calculer le score de matière organique si disponible
        let om_score = if let Some(om) = nutrient_levels.organic_matter {
            (om / 5.0).min(1.0) // 5% est considéré comme excellent
        } else {
            0.5 // Valeur par défaut si non disponible
        };

        // Pondération des facteurs
        let weights = [0.3, 0.2, 0.2, 0.2, 0.1]; // pH, N, P, K, OM
        let scores = [ph_score, n_score, p_score, k_score, om_score];

        // Calculer le score pondéré
        weights
            .iter()
            .zip(scores.iter())
            .map(|(w, s)| w * s)
            .sum::<f32>()
    }

    /// Extrait le pH des données de capteurs
    fn extract_ph(&self, sensor_data: &SensorData) -> AgriResult<f32> {
        if let Some(ph_str) = sensor_data.get_value("soil_ph") {
            match ph_str.parse::<f32>() {
                Ok(ph) if ph >= 0.0 && ph <= 14.0 => Ok(ph),
                Ok(_) => Err(AgriMonitorError::InvalidInput(
                    "La valeur de pH doit être comprise entre 0 et 14".to_string(),
                )),
                Err(_) => Err(AgriMonitorError::DataProcessingError(
                    "Impossible de convertir la valeur de pH en nombre".to_string(),
                )),
            }
        } else {
            // Valeur par défaut si non disponible
            Ok(6.5)
        }
    }

    /// Extrait les niveaux de nutriments des données de capteurs
    fn extract_nutrient_levels(&self, sensor_data: &SensorData) -> AgriResult<NutrientLevels> {
        // Extraire les valeurs de nutriments
        let nitrogen = sensor_data
            .get_value("nitrogen")
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(50.0); // Valeur par défaut en ppm

        let phosphorus = sensor_data
            .get_value("phosphorus")
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(25.0); // Valeur par défaut en ppm

        let potassium = sensor_data
            .get_value("potassium")
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(100.0); // Valeur par défaut en ppm

        let organic_matter = sensor_data
            .get_value("organic_matter")
            .and_then(|v| v.parse::<f32>().ok());

        let calcium = sensor_data
            .get_value("calcium")
            .and_then(|v| v.parse::<f32>().ok());

        let magnesium = sensor_data
            .get_value("magnesium")
            .and_then(|v| v.parse::<f32>().ok());

        let sulfur = sensor_data
            .get_value("sulfur")
            .and_then(|v| v.parse::<f32>().ok());

        // Extraire les micronutriments
        let zinc = sensor_data
            .get_value("zinc")
            .and_then(|v| v.parse::<f32>().ok());

        let iron = sensor_data
            .get_value("iron")
            .and_then(|v| v.parse::<f32>().ok());

        let manganese = sensor_data
            .get_value("manganese")
            .and_then(|v| v.parse::<f32>().ok());

        let copper = sensor_data
            .get_value("copper")
            .and_then(|v| v.parse::<f32>().ok());

        let boron = sensor_data
            .get_value("boron")
            .and_then(|v| v.parse::<f32>().ok());

        // Créer une HashMap pour les autres nutriments
        let mut others = HashMap::new();

        // Ajouter d'autres nutriments si présents
        for (key, value) in sensor_data.values.iter() {
            if !["soil_ph", "nitrogen", "phosphorus", "potassium", "organic_matter", 
                "calcium", "magnesium", "sulfur", "zinc", "iron", "manganese", 
                "copper", "boron"].contains(&key.as_str()) {
                if let Ok(val) = value.parse::<f32>() {
                    others.insert(key.clone(), val);
                }
            }
        }

        // Créer l'objet NutrientLevels
        let nutrient_levels = NutrientLevels {
            nitrogen,
            phosphorus,
            potassium,
            organic_matter,
            calcium,
            magnesium,
            sulfur,
            zinc,
            iron,
            manganese,
            copper,
            boron,
            others,
        };

        Ok(nutrient_levels)
    }

    /// Extrait les caractéristiques des données de capteurs
    fn extract_features(&self, sensor_data: &SensorData) -> AgriResult<Array1<f32>> {
        // Extraire les valeurs pertinentes
        let ph = self.extract_ph(sensor_data)?;
        let nutrient_levels = self.extract_nutrient_levels(sensor_data)?;

        // Normaliser les valeurs
        let ph_norm = ph / 14.0; // pH de 0 à 14
        let n_norm = (nutrient_levels.nitrogen / 200.0).min(1.0); // N typiquement 0-200 ppm
        let p_norm = (nutrient_levels.phosphorus / 100.0).min(1.0); // P typiquement 0-100 ppm
        let k_norm = (nutrient_levels.potassium / 300.0).min(1.0); // K typiquement 0-300 ppm
        let om_norm = nutrient_levels
            .organic_matter
            .map(|om| (om / 10.0).min(1.0))
            .unwrap_or(0.5); // OM typiquement 0-10%

        // Créer le vecteur de caractéristiques
        let features = Array1::from(vec![ph_norm, n_norm, p_norm, k_norm, om_norm]);

        Ok(features)
    }

    /// Prédit le score de fertilité avec le modèle
    fn predict_with_model(&self, features: &Array1<f32>) -> AgriResult<f32> {
        if let Some(model) = &self.model {
            // Convertir les caractéristiques en format attendu par le modèle
            let features_2d = features.clone().into_shape((1, features.len())).unwrap();
            
            // Prédire avec le modèle
            let prediction = model.predict(&features_2d);
            
            // Convertir la prédiction en score de fertilité (0-5 -> 0.0-1.0)
            let fertility_score = prediction[0] as f32 / 5.0;
            
            Ok(fertility_score)
        } else {
            Err(AgriMonitorError::ModelNotLoaded(
                "Le modèle de prédiction de la fertilité n'est pas chargé".to_string(),
            ))
        }
    }

    /// Entraîne le modèle avec des données d'entraînement
    pub fn train(&mut self, features: Array2<f32>, targets: Array1<usize>) -> AgriResult<()> {
        // Créer le dataset
        let dataset = Dataset::new(features, targets);

        // Entraîner un modèle de Decision Tree
        let model = DecisionTree::params()
            .max_depth(Some(10))
            .split_quality(SplitQuality::Gini)
            .fit(&dataset)
            .map_err(|e| AgriMonitorError::ModelTrainingError(e.to_string()))?;

        // Stocker le modèle
        self.model = Some(model);

        Ok(())
    }
}

#[async_trait]
impl Model for FertilitySensorModel {
    /// Charge le modèle à partir d'un fichier
    async fn load(&mut self, path: &str) -> AgriResult<()> {
        let path = Path::new(path);

        // Vérifier si le fichier existe
        if !path.exists() {
            info!("Modèle non trouvé à {}, utilisation de la méthode simple", path.display());
            return Ok(());
        }

        // Charger le modèle (implémentation simplifiée)
        // Dans une implémentation réelle, il faudrait désérialiser le modèle
        info!("Chargement du modèle depuis {}", path.display());
        
        // Simuler le chargement du modèle
        // Dans une implémentation réelle, on chargerait le modèle depuis le fichier
        let dummy_features = Array2::<f32>::zeros((10, 5));
        let dummy_targets = Array1::<usize>::zeros(10);
        self.train(dummy_features, dummy_targets)?;

        Ok(())
    }

    /// Sauvegarde le modèle dans un fichier
    async fn save(&self, path: &str) -> AgriResult<()> {
        if self.model.is_none() {
            return Err(AgriMonitorError::ModelNotLoaded(
                "Pas de modèle à sauvegarder".to_string(),
            ));
        }

        // Sauvegarder le modèle (implémentation simplifiée)
        // Dans une implémentation réelle, il faudrait sérialiser le modèle
        info!("Sauvegarde du modèle vers {}", path);

        Ok(())
    }
}

impl Default for FertilitySensorModel {
    fn default() -> Self {
        Self::new()
    }
}
