//! Tests pour le module de fertilité
//!
//! Ce module contient les tests unitaires pour le module de fertilité.

#[cfg(test)]
mod tests {
    use crate::types::{
        CropFertilityData, FertilityPrediction, FertilizationPriority, NutrientLevels, SoilData,
        SoilFertilityStatus,
    };
    use crate::utils;
    use common::models::Model;
    use chrono::Utc;
    use std::collections::HashMap;

    /// Crée une prédiction de fertilité pour les tests
    fn create_test_prediction() -> FertilityPrediction {
        FertilityPrediction {
            timestamp: Utc::now(),
            fertility_score: 0.7,
            status: SoilFertilityStatus::Good,
            ph_level: 6.5,
            nutrient_levels: NutrientLevels {
                nitrogen: 80.0,
                phosphorus: 40.0,
                potassium: 150.0,
                organic_matter: Some(3.5),
                calcium: Some(1000.0),
                magnesium: Some(200.0),
                sulfur: Some(15.0),
                zinc: Some(2.0),
                iron: Some(15.0),
                manganese: Some(10.0),
                copper: Some(1.0),
                boron: Some(0.5),
                others: HashMap::new(),
            },
            confidence: 0.85,
        }
    }

    /// Crée des données de sol pour les tests
    fn create_test_soil_data() -> SoilData {
        SoilData {
            soil_type: "Loam".to_string(),
            soil_texture: "Medium".to_string(),
            cec: Some(15.0),
            bulk_density: Some(1.3),
            soil_depth_cm: 30.0,
        }
    }

    /// Crée des données de culture pour les tests
    fn create_test_crop_data() -> CropFertilityData {
        let mut other_requirements = HashMap::new();
        other_requirements.insert("zinc".to_string(), 3.0);
        other_requirements.insert("boron".to_string(), 1.0);

        CropFertilityData {
            crop_type: "Maize".to_string(),
            growth_stage: "vegetative".to_string(),
            nitrogen_requirement: 120.0,
            phosphorus_requirement: 60.0,
            potassium_requirement: 180.0,
            optimal_ph_range: (5.8, 7.0),
            other_requirements,
        }
    }

    #[test]
    fn test_calculate_fertility_score() {
        let ph = 6.5;
        let nutrient_levels = NutrientLevels {
            nitrogen: 80.0,
            phosphorus: 40.0,
            potassium: 150.0,
            organic_matter: Some(3.5),
            calcium: None,
            magnesium: None,
            sulfur: None,
            zinc: None,
            iron: None,
            manganese: None,
            copper: None,
            boron: None,
            others: HashMap::new(),
        };
        let crop_data = create_test_crop_data();

        let result = utils::calculate_fertility_score(ph, &nutrient_levels, &crop_data);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_calculate_fertilization_recommendation() {
        let prediction = create_test_prediction();
        let soil_data = create_test_soil_data();
        let crop_data = create_test_crop_data();

        let result = utils::calculate_fertilization_recommendation(&prediction, &soil_data, &crop_data);
        assert!(result.is_ok());

        let recommendation = result.unwrap();
        assert!(recommendation.fertilizer_amounts.contains_key("nitrogen"));
        assert!(recommendation.fertilizer_amounts.contains_key("phosphorus"));
        assert!(recommendation.fertilizer_amounts.contains_key("potassium"));
    }

    #[test]
    fn test_ndvi_to_fertility_score() {
        // Test avec différentes valeurs d'NDVI
        let ndvi_values = [-0.6, -0.2, 0.1, 0.3, 0.5, 0.7];
        let expected_ranges = [
            (0.0, 0.1),   // NDVI < -0.5
            (0.1, 0.2),   // NDVI < 0.0
            (0.2, 0.4),   // NDVI < 0.2
            (0.4, 0.6),   // NDVI < 0.4
            (0.6, 0.8),   // NDVI < 0.6
            (0.8, 1.0),   // NDVI >= 0.6
        ];

        for (i, ndvi) in ndvi_values.iter().enumerate() {
            let score = utils::ndvi_to_fertility_score(*ndvi);
            let (min, max) = expected_ranges[i];
            assert!(
                score >= min && score <= max,
                "NDVI {} devrait donner un score entre {} et {}, mais a donné {}",
                ndvi, min, max, score
            );
        }
    }

    #[test]
    fn test_calculate_nutrient_deficit() {
        // Test avec un niveau actuel inférieur au besoin
        let current_level = 80.0;
        let requirement = 120.0;
        let deficit = utils::calculate_nutrient_deficit(current_level, requirement);
        assert_eq!(deficit, 40.0);

        // Test avec un niveau actuel supérieur au besoin
        let current_level = 150.0;
        let requirement = 120.0;
        let deficit = utils::calculate_nutrient_deficit(current_level, requirement);
        assert_eq!(deficit, 0.0);
    }

    #[test]
    fn test_determine_fertilization_priority() {
        let mut prediction = create_test_prediction();
        let crop_data = create_test_crop_data();

        // Test avec des niveaux de nutriments élevés
        prediction.nutrient_levels.nitrogen = 100.0;
        prediction.nutrient_levels.phosphorus = 50.0;
        prediction.nutrient_levels.potassium = 160.0;
        let priority = utils::determine_fertilization_priority(&prediction, &crop_data);
        assert_eq!(priority, FertilizationPriority::Low);

        // Test avec des niveaux de nutriments très bas
        prediction.nutrient_levels.nitrogen = 30.0;
        prediction.nutrient_levels.phosphorus = 15.0;
        prediction.nutrient_levels.potassium = 50.0;
        let priority = utils::determine_fertilization_priority(&prediction, &crop_data);
        assert_eq!(priority, FertilizationPriority::High);
    }

    #[test]
    fn test_calculate_ndvi() {
        let red_band = vec![0.1, 0.2, 0.3, 0.4];
        let nir_band = vec![0.5, 0.6, 0.7, 0.8];

        let result = utils::calculate_ndvi(&red_band, &nir_band);
        assert!(result.is_ok());

        let ndvi = result.unwrap();
        assert_eq!(ndvi.len(), 4);

        // Vérifier les valeurs NDVI
        // NDVI = (NIR - RED) / (NIR + RED)
        assert!((ndvi[0] - 0.6666667).abs() < 0.0001); // (0.5 - 0.1) / (0.5 + 0.1)
        assert!((ndvi[1] - 0.5).abs() < 0.0001);       // (0.6 - 0.2) / (0.6 + 0.2)
        assert!((ndvi[2] - 0.4).abs() < 0.0001);       // (0.7 - 0.3) / (0.7 + 0.3)
        assert!((ndvi[3] - 0.3333333).abs() < 0.0001); // (0.8 - 0.4) / (0.8 + 0.4)
    }
}
