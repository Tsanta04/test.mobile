use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::sensor_type::{CreateSensorTypeDto, SensorTypeResponseDto, UpdateSensorTypeDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::SensorTypeService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_sensor_type(
    pool: web::Data<PgPool>,
    sensor_type_dto: web::Json<CreateSensorTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the sensor type DTO
    sensor_type_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can create sensor types".to_string()));
    }

    // Create the sensor type
    let sensor_type = SensorTypeService::create_sensor_type(&pool, sensor_type_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        SensorTypeResponseDto::from(sensor_type),
        "Sensor type created successfully",
    )))
}

pub async fn get_sensor_type_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sensor_type_id = path.into_inner();

    // Get the sensor type
    let sensor_type = SensorTypeService::get_sensor_type_by_id(&pool, sensor_type_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        SensorTypeResponseDto::from(sensor_type),
        "Sensor type retrieved successfully",
    )))
}

pub async fn get_all_sensor_types(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Get all sensor types
    let sensor_types = SensorTypeService::get_all_sensor_types(&pool).await?;

    // Convert to response DTOs
    let sensor_type_dtos: Vec<SensorTypeResponseDto> = sensor_types
        .into_iter()
        .map(SensorTypeResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_type_dtos,
        "Sensor types retrieved successfully",
    )))
}

pub async fn update_sensor_type(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    sensor_type_dto: web::Json<UpdateSensorTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_type_id = path.into_inner();

    // Validate the sensor type DTO
    sensor_type_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can update sensor types".to_string()));
    }

    // Update the sensor type
    let sensor_type = SensorTypeService::update_sensor_type(&pool, sensor_type_id, sensor_type_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        SensorTypeResponseDto::from(sensor_type),
        "Sensor type updated successfully",
    )))
}

pub async fn delete_sensor_type(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_type_id = path.into_inner();

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete sensor types".to_string()));
    }

    // Delete the sensor type
    SensorTypeService::delete_sensor_type(&pool, sensor_type_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Sensor type deleted successfully",
    )))
}

