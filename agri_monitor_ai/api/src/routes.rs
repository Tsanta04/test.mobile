//! API routes
//!
//! This module configures the API routes.

use crate::handlers::disease_detection::{detect_disease_image, detect_disease_sensor, health_check, DiseaseDetectionState};
use crate::AppState;
use actix_web::web;
use std::sync::Arc;

/// Configure API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    // Create disease detection state
    let disease_detection_state = web::Data::new(
        tokio::runtime::Handle::current().block_on(async {
            DiseaseDetectionState::new("./models").await
        })
    );
    
    cfg.app_data(disease_detection_state.clone())
        .service(
            web::scope("/api/v1")
                // Health check
                .route("/health", web::get().to(health_check))
                
                // Disease detection
                .service(
                    web::scope("/disease-detection")
                        .route("/sensor", web::post().to(detect_disease_sensor))
                        .route("/image", web::post().to(detect_disease_image))
                )
        );
}

