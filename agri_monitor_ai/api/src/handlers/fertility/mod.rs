//! Handlers pour les prédictions de fertilité du sol
//!
//! Ce module contient les handlers pour les endpoints de l'API liés à la fertilité du sol.

use actix_web::{web, HttpResponse, Responder};
use common::error::AgriResult;
use serde_json::json;
use fertility::{
    FertilityPredictor,
    types::{SensorPredictionRequest, ImagePredictionRequest},
};

/// État partagé pour les prédictions de fertilité
pub struct FertilityState {
    predictor: FertilityPredictor,
}

impl FertilityState {
    /// Crée un nouvel état pour les prédictions de fertilité
    pub async fn new(models_path: &str) -> Self {
        let mut predictor = FertilityPredictor::new();
        
        // Charger les modèles
        let sensor_model_path = format!("{}/fertility_sensor.model", models_path);
        let image_model_path = format!("{}/fertility_image.model", models_path);
        
        if let Err(e) = predictor.load_models(&sensor_model_path, &image_model_path).await {
            tracing::warn!("Erreur lors du chargement des modèles de fertilité: {}", e);
            tracing::info!("Utilisation des méthodes de prédiction simples");
        }
        
        Self { predictor }
    }
}

/// Prédit la fertilité à partir de données de capteurs
pub async fn predict_from_sensor(
    state: web::Data<FertilityState>,
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
            tracing::error!("Erreur lors de la prédiction de la fertilité: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Erreur lors de la prédiction: {}", e),
            }))
        }
    }
}

/// Prédit la fertilité à partir d'une image
pub async fn predict_from_image(
    state: web::Data<FertilityState>,
    request: web::Json<ImagePredictionRequest>,
) -> impl Responder {
    match state.predictor.predict_from_image(request.into_inner()).await {
        Ok((fertility_map, recommendation)) => {
            HttpResponse::Ok().json(json!({
                "fertility_map": fertility_map,
                "recommendation": recommendation,
            }))
        }
        Err(e) => {
            tracing::error!("Erreur lors de la prédiction de la fertilité: {}", e);
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
                "fertility_score": 0.75,
                "status": "good",
                "ph_level": 6.5,
                "nutrient_levels": {
                    "nitrogen": 85.0,
                    "phosphorus": 45.0,
                    "potassium": 160.0,
                    "organic_matter": 3.8
                }
            },
            {
                "timestamp": "2023-07-17T10:00:00Z",
                "fertility_score": 0.68,
                "status": "good",
                "ph_level": 6.3,
                "nutrient_levels": {
                    "nitrogen": 78.0,
                    "phosphorus": 42.0,
                    "potassium": 155.0,
                    "organic_matter": 3.5
                }
            },
            {
                "timestamp": "2023-06-17T10:00:00Z",
                "fertility_score": 0.55,
                "status": "moderate",
                "ph_level": 6.1,
                "nutrient_levels": {
                    "nitrogen": 65.0,
                    "phosphorus": 35.0,
                    "potassium": 140.0,
                    "organic_matter": 3.2
                }
            },
        ],
    }))
}

/// Récupère les recommandations de fertilisation pour un champ
pub async fn get_recommendations(
    path: web::Path<String>,
) -> impl Responder {
    let field_id = path.into_inner();
    
    // Dans une implémentation réelle, on récupérerait les recommandations depuis la base de données
    // Pour cet exemple, on renvoie une recommandation factice
    
    HttpResponse::Ok().json(json!({
        "field_id": field_id,
        "recommendations": {
            "fertilization_needed": true,
            "priority": "medium",
            "fertilizer_amounts": {
                "nitrogen": 40.0,
                "phosphorus": 20.0,
                "potassium": 30.0
            },
            "recommended_fertilizer_type": "Engrais complet NPK équilibré",
            "optimal_time": "Pendant la phase de croissance végétative",
            "notes": [
                "Statut de fertilité du sol: good.",
                "Le pH du sol (6.5) est optimal pour maize.",
                "Type de sol: Loam (Medium). Adapter les pratiques de fertilisation en conséquence."
            ],
        },
    }))
}

