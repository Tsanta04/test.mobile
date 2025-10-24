//! Configuration des routes de l'API
//!
//! Ce module définit toutes les routes de l'API et leur configuration.

use crate::handlers::{
    disease_detection::{detect_disease_image as disease_predict_image, detect_disease_sensor as disease_predict_sensor},
    water_level::{predict_from_image as water_predict_image, predict_from_sensor as water_predict_sensor, get_history as water_get_history, get_recommendations as water_get_recommendations},
    fertility::{predict_from_image as fertility_predict_image, predict_from_sensor as fertility_predict_sensor, get_history as fertility_get_history, get_recommendations as fertility_get_recommendations},
};
use actix_web::web;

/// Configure toutes les routes de l'API
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Routes pour la détection de maladies
            .service(
                web::scope("/disease-detection")
                    .route("/predict/sensor", web::post().to(disease_predict_sensor))
                    .route("/predict/image", web::post().to(disease_predict_image))
            )
            // Routes pour le taux d'eau
            .service(
                web::scope("/water-level")
                    .route("/predict/sensor", web::post().to(water_predict_sensor))
                    .route("/predict/image", web::post().to(water_predict_image))
                    .route("/history/{field_id}", web::get().to(water_get_history))
                    .route("/recommendations/{field_id}", web::get().to(water_get_recommendations))
            )
            // Routes pour la fertilité du sol
            .service(
                web::scope("/fertility")
                    .route("/predict/sensor", web::post().to(fertility_predict_sensor))
                    .route("/predict/image", web::post().to(fertility_predict_image))
                    .route("/history/{field_id}", web::get().to(fertility_get_history))
                    .route("/recommendations/{field_id}", web::get().to(fertility_get_recommendations))
            )
    );
}
