//! Tests pour le module de prédiction du taux d'eau

use crate::{
    types::*,
    utils,
    WaterLevelPredictor,
};
use chrono::Utc;
use common::data::{GeoLocation, ImageData, SensorData};
use std::collections::HashMap;

#[tokio::test]
async fn test_sensor_prediction() {
    // Créer un prédicteur
    let predictor = WaterLevelPredictor::new();

    // Créer des données de test
    let mut sensor_data = SensorData {
        timestamp: Utc::now(),
        location: GeoLocation {
            latitude: 48.8566,
            longitude: 2.3522,
        },
        values: HashMap::new(),
    };

    // Ajouter des valeurs de capteurs
    sensor_data.values.insert("soil_moisture".to_string(), "45".to_string());
    sensor_data.values.insert("air_humidity".to_string(), "65".to_string());
    sensor_data.values.insert("temperature".to_string(), "25".to_string());

    // Créer des données météorologiques
    let weather_data = WeatherData {
        temperature: 25.0,
        air_humidity: 0.65,
        forecast_precipitation_24h: 0.0,
        wind_speed: Some(3.0),
        evapotranspiration: Some(5.0),
    };

    // Créer des données de culture
    let crop_data = CropData {
        crop_type: "wheat".to_string(),
        growth_stage: "mature".to_string(),
        crop_coefficient: 0.8,
        root_depth_cm: 30.0,
        critical_moisture_threshold: 0.4,
    };

    // Créer la requête
    let request = SensorPredictionRequest {
        sensor_data,
        weather_data,
        crop_data,
        field_id: Some("test_field".to_string()),
    };

    // Effectuer la prédiction
    let result = predictor.predict_from_sensor_data(request).await;

    // Vérifier le résultat
    assert!(result.is_ok());
    let (prediction, recommendation) = result.unwrap();

    // Vérifier la prédiction
    assert!(prediction.water_level >= 0.0 && prediction.water_level <= 1.0);
    assert!(prediction.confidence >= 0.0 && prediction.confidence <= 1.0);

    // Vérifier la recommandation
    assert!(recommendation.water_amount_mm >= 0.0);
}

#[tokio::test]
async fn test_image_prediction() {
    // Créer un prédicteur
    let predictor = WaterLevelPredictor::new();

    // Créer des données d'image de test
    let image_data = ImageData {
        timestamp: Utc::now(),
        location: GeoLocation {
            latitude: 48.8566,
            longitude: 2.3522,
        },
        image: vec![0u8; 100 * 100 * 3], // Image RGB factice 100x100
        image_type: "jpeg".to_string(),
    };

    // Créer des données météorologiques
    let weather_data = WeatherData {
        temperature: 25.0,
        air_humidity: 0.65,
        forecast_precipitation_24h: 0.0,
        wind_speed: Some(3.0),
        evapotranspiration: Some(5.0),
    };

    // Créer des données de culture
    let crop_data = CropData {
        crop_type: "wheat".to_string(),
        growth_stage: "mature".to_string(),
        crop_coefficient: 0.8,
        root_depth_cm: 30.0,
        critical_moisture_threshold: 0.4,
    };

    // Créer la requête
    let request = ImagePredictionRequest {
        image_data,
        weather_data,
        crop_data,
        field_id: Some("test_field".to_string()),
    };

    // Effectuer la prédiction
    let result = predictor.predict_from_image(request).await;

    // Vérifier le résultat
    assert!(result.is_ok());
    let (water_map, recommendation) = result.unwrap();

    // Vérifier la carte d'humidité
    assert!(!water_map.water_level_matrix.is_empty());
    assert!(water_map.average_water_level >= 0.0 && water_map.average_water_level <= 1.0);

    // Vérifier la recommandation
    assert!(recommendation.water_amount_mm >= 0.0);
}

#[test]
fn test_soil_moisture_status() {
    // Tester la conversion de pourcentage en statut
    assert!(matches!(SoilMoistureStatus::from_percentage(0.1), SoilMoistureStatus::Dry));
    assert!(matches!(SoilMoistureStatus::from_percentage(0.3), SoilMoistureStatus::SlightlyDry));
    assert!(matches!(SoilMoistureStatus::from_percentage(0.5), SoilMoistureStatus::Optimal));
    assert!(matches!(SoilMoistureStatus::from_percentage(0.8), SoilMoistureStatus::Wet));
    assert!(matches!(SoilMoistureStatus::from_percentage(0.95), SoilMoistureStatus::Saturated));
}

#[test]
fn test_irrigation_recommendation() {
    // Créer une prédiction de test
    let prediction = WaterLevelPrediction {
        timestamp: Utc::now(),
        water_level: 0.3,
        status: SoilMoistureStatus::SlightlyDry,
        confidence: 0.8,
    };

    // Créer des données de culture
    let crop_data = CropData {
        crop_type: "wheat".to_string(),
        growth_stage: "mature".to_string(),
        crop_coefficient: 0.8,
        root_depth_cm: 30.0,
        critical_moisture_threshold: 0.4,
    };

    // Créer des données météorologiques
    let weather_data = WeatherData {
        temperature: 25.0,
        air_humidity: 0.65,
        forecast_precipitation_24h: 0.0,
        wind_speed: Some(3.0),
        evapotranspiration: Some(5.0),
    };

    // Calculer la recommandation
    let result = utils::calculate_irrigation_recommendation(&prediction, &crop_data, &weather_data);

    // Vérifier le résultat
    assert!(result.is_ok());
    let recommendation = result.unwrap();

    // Vérifier que l'irrigation est recommandée (car niveau d'eau < seuil critique)
    assert!(recommendation.irrigation_needed);
    assert!(recommendation.water_amount_mm > 0.0);
}

#[test]
fn test_ndvi_conversion() {
    // Tester la conversion NDVI -> taux d'humidité
    let ndvi_values = vec![-0.5, 0.0, 0.3, 0.6, 0.8];
    
    for ndvi in ndvi_values {
        let water_level = utils::ndvi_to_water_level(ndvi);
        
        // Vérifier que le résultat est dans l'intervalle [0,1]
        assert!(water_level >= 0.0 && water_level <= 1.0);
        
        // Vérifier la relation NDVI/humidité
        if ndvi > 0.6 {
            assert!(water_level > 0.6); // NDVI élevé -> humidité élevée
        } else if ndvi < 0.0 {
            assert!(water_level < 0.4); // NDVI négatif -> humidité faible
        }
    }
}

#[test]
fn test_matrix_operations() {
    // Créer une matrice de test
    let mut matrix = vec![
        vec![0.1, 0.2, 0.3],
        vec![0.4, 0.5, 0.6],
        vec![0.7, 0.8, 0.9],
    ];

    // Tester le calcul de moyenne
    let average = utils::calculate_matrix_average(&matrix);
    assert!((average - 0.5).abs() < 0.001);

    // Tester la normalisation
    utils::normalize_matrix(&mut matrix);

    // Vérifier que les valeurs sont entre 0 et 1
    for row in &matrix {
        for &value in row {
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    // Vérifier que le min est 0 et le max est 1
    let mut min_val: f32 = 1.0;
    let mut max_val: f32 = 0.0;
    for row in &matrix {
        for &value in row {
            min_val = min_val.min(value);
            max_val = max_val.max(value);
        }
    }
    assert!((min_val - 0.0).abs() < 0.001);
    assert!((max_val - 1.0).abs() < 0.001);
}
