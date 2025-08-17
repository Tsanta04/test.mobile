//! Handlers pour les prédictions de taux d'eau
//!
//! Ce module contient les handlers pour les endpoints de l'API liés au taux d'eau.

use actix_web::{web, HttpResponse, Responder};
use common::error::AgriResult;
use serde::{Deserialize, Serialize};
use water_level::{
    WaterLevelPredictor,
    types::{SensorPredictionRequest, ImagePredictionRequest},
};

/// État partagé pour les prédictions de taux d'eau
pub struct WaterLevelState {
    predictor: WaterLevelPredictor,
}

impl WaterLevelState {
    /// Crée un nouvel état pour les prédictions de taux d'eau
    pub async fn new(models_path: &str) -> Self {
        let mut predictor = WaterLevelPredictor::new();
        
        // Charger les modèles
        let sensor_model_path = format!("{}/water_level_sensor.model", models_path);
        let image_model_path = format!("{}/water_level_image.model", models_path);
        
        if let Err(e) = predictor.load_models(&sensor_model_path, &image_model_path).await {
            tracing::warn!("Erreur lors du chargement des modèles de taux d'eau: {}", e);
            tracing::info!("Utilisation des méthodes de prédiction simples");
        }
        
        Self { predictor }
    }
}

/// Prédit le taux d'eau à partir de données de capteurs
pub async fn predict_from_sensor(
    state: web::Data<WaterLevelState>,
    request: web::Json<SensorPredictionRequest>,
) -> impl Responder {
    match state.predictor.predict_from_sensor_data(request.into_inner()).await {
        Ok((prediction, recommendation)) => {
            HttpResponse::Ok().json(json!({
                "prediction": prediction,
                "recommendation": recommendation,
            }))
        }
        Err(e) => {
            tracing::error!("Erreur lors de la prédiction du taux d'eau: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Erreur lors de la prédiction: {}", e),
            }))
        }
    }
}

/// Prédit le taux d'eau à partir d'une image
pub async fn predict_from_image(
    state: web::Data<WaterLevelState>,
    request: web::Json<ImagePredictionRequest>,
) -> impl Responder {
    match state.predictor.predict_from_image(request.into_inner()).await {
        Ok((water_map, recommendation)) => {
            HttpResponse::Ok().json(json!({
                "water_map": water_map,
                "recommendation": recommendation,
            }))
        }
        Err(e) => {
            tracing::error!("Erreur lors de la prédiction du taux d'eau: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Erreur lors de la prédiction: {}", e),
            }))
        }
    }
}

/// Récupère l'historique des prédictions pour un champ
pub async fn get_history(
    path: web::Path<String>,
) -> impl Responder {
    let field_id = path.into_inner();
    
    // Dans une implémentation réelle, on récupérerait l'historique depuis la base de données
    // Pour cet exemple, on renvoie un historique factice
    
    HttpResponse::Ok().json(json!({
        "field_id": field_id,
        "history": [
            {
                "timestamp": "2023-08-17T10:00:00Z",
                "water_level": 0.65,
                "status": "optimal",
            },
            {
                "timestamp": "2023-08-16T10:00:00Z",
                "water_level": 0.58,
                "status": "optimal",
            },
            {
                "timestamp": "2023-08-15T10:00:00Z",
                "water_level": 0.45,
                "status": "slightly_dry",
            },
        ],
    }))
}

/// Récupère les recommandations d'arrosage pour un champ
pub async fn get_recommendations(
    path: web::Path<String>,
) -> impl Responder {
    let field_id = path.into_inner();
    
    // Dans une implémentation réelle, on récupérerait les recommandations depuis la base de données
    // Pour cet exemple, on renvoie une recommandation factice
    
    HttpResponse::Ok().json(json!({
        "field_id": field_id,
        "recommendations": {
            "irrigation_needed": true,
            "water_amount_mm": 15.5,
            "urgency": "medium",
            "optimal_time": "morning",
            "notes": [
                "Le sol est légèrement sec, irrigation recommandée.",
                "Précipitations de 2.5 mm prévues dans les prochaines 24h.",
                "Stade de croissance actuel: mature. Besoin en eau typique: 4.0 mm/jour.",
            ],
        },
    }))
}

