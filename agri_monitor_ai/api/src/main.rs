//! Agricultural Monitoring AI API
//!
//! This is the main entry point for the API that provides access to
//! all prediction modules for agricultural monitoring.

mod config;
mod handlers;
mod models;
mod routes;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
// use std::sync::Arc; // Commenté car non utilisé
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// Application state shared across all routes
pub struct AppState {
    /// Configuration
    pub config: config::Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    // Load configuration
    let config = config::Config::from_env();
    let host = config.host.clone();
    let port = config.port;
    info!("Starting server at {}:{}", host, port);
    
    // Create shared application state
    let app_state = web::Data::new(AppState { config });
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .configure(routes::configure_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
