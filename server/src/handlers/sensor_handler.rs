use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::sensor::{CreateSensorDto, SensorResponseDto, UpdateSensorDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::SensorService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_sensor(
    pool: web::Data<PgPool>,
    sensor_dto: web::Json<CreateSensorDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the sensor DTO
    sensor_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can create sensors".to_string()));
    }

    // Create the sensor
    let sensor = SensorService::create_sensor(&pool, sensor_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        SensorResponseDto::from(sensor),
        "Sensor created successfully",
    )))
}

pub async fn get_sensor_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();

    // Get the sensor
    let sensor = SensorService::get_sensor_by_id(&pool, sensor_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        SensorResponseDto::from(sensor),
        "Sensor retrieved successfully",
    )))
}

pub async fn get_sensors_by_type(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sensor_type_id = path.into_inner();

    // Get the sensors
    let sensors = SensorService::get_sensors_by_type(&pool, sensor_type_id).await?;

    // Convert to response DTOs
    let sensor_dtos: Vec<SensorResponseDto> = sensors
        .into_iter()
        .map(SensorResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_dtos,
        "Sensors retrieved successfully",
    )))
}

pub async fn get_all_sensors(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Get all sensors
    let sensors = SensorService::get_all_sensors(&pool).await?;

    // Convert to response DTOs
    let sensor_dtos: Vec<SensorResponseDto> = sensors
        .into_iter()
        .map(SensorResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_dtos,
        "Sensors retrieved successfully",
    )))
}

pub async fn update_sensor(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    sensor_dto: web::Json<UpdateSensorDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();

    // Validate the sensor DTO
    sensor_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can update sensors".to_string()));
    }

    // Update the sensor
    let sensor = SensorService::update_sensor(&pool, sensor_id, sensor_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        SensorResponseDto::from(sensor),
        "Sensor updated successfully",
    )))
}

pub async fn delete_sensor(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete sensors".to_string()));
    }

    // Delete the sensor
    SensorService::delete_sensor(&pool, sensor_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Sensor deleted successfully",
    )))
}

