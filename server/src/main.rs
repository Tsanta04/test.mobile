#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

mod dto;
mod errors;
mod handlers;
mod models;
mod routes;
mod schema;
mod services;

use services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    let pool = Arc::new(pool);

    // Create service instances
    let users_service = Arc::new(UsersService::new(pool.clone()));
    let person_service = Arc::new(PersonService::new(pool.clone()));
    let culture_type_service = Arc::new(CultureTypeService::new(pool.clone()));
    let location_service = Arc::new(LocationService::new(pool.clone()));
    let sensor_type_service = Arc::new(SensorTypeService::new(pool.clone()));
    let sensor_service = Arc::new(SensorService::new(pool.clone()));
    let sensor_pack_service = Arc::new(SensorPackService::new(pool.clone()));
    let pack_service = Arc::new(PackService::new(pool.clone()));
    let ground_service = Arc::new(GroundService::new(pool.clone()));
    let discussion_service = Arc::new(DiscussionService::new(pool.clone()));
    let message_service = Arc::new(MessageService::new(pool.clone()));
    let participant_service = Arc::new(ParticipantService::new(pool.clone()));
    let state_service = Arc::new(StateService::new(pool.clone()));
    let alert_service = Arc::new(AlertService::new(pool.clone()));
    let planning_service = Arc::new(PlanningService::new(pool.clone()));

    println!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(handlers::Authentication)
            .app_data(web::Data::new(users_service.clone()))
            .app_data(web::Data::new(person_service.clone()))
            .app_data(web::Data::new(culture_type_service.clone()))
            .app_data(web::Data::new(location_service.clone()))
            .app_data(web::Data::new(sensor_type_service.clone()))
            .app_data(web::Data::new(sensor_service.clone()))
            .app_data(web::Data::new(sensor_pack_service.clone()))
            .app_data(web::Data::new(pack_service.clone()))
            .app_data(web::Data::new(ground_service.clone()))
            .app_data(web::Data::new(discussion_service.clone()))
            .app_data(web::Data::new(message_service.clone()))
            .app_data(web::Data::new(participant_service.clone()))
            .app_data(web::Data::new(state_service.clone()))
            .app_data(web::Data::new(alert_service.clone()))
            .app_data(web::Data::new(planning_service.clone()))
            .configure(routes::configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

