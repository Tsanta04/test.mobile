//! Configuration des routes de l'API
//!
//! Ce module définit toutes les routes de l'API et leur configuration.

use crate::handlers::{
    disease_detection::{predict_from_image as disease_predict_image, predict_from_sensor as disease_predict_sensor},
    water_level::{predict_from_image as water_predict_image, predict_from_sensor as water_predict_sensor, get_history, get_recommendations},
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
                    .route("/history/{field_id}", web::get().to(get_history))
                    .route("/recommendations/{field_id}", web::get().to(get_recommendations))
            )
    );
}

