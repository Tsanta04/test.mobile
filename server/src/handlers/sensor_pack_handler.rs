use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::sensor_pack::{CreateSensorPackDto, SensorPackResponseDto, UpdateSensorPackDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::SensorPackService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_sensor_pack(
    pool: web::Data<PgPool>,
    sensor_pack_dto: web::Json<CreateSensorPackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the sensor pack DTO
    sensor_pack_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can create sensor packs".to_string()));
    }

    // Create the sensor pack
    let sensor_pack = SensorPackService::create_sensor_pack(&pool, sensor_pack_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        SensorPackResponseDto::from(sensor_pack),
        "Sensor pack created successfully",
    )))
}

pub async fn get_sensor_pack_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();

    // Get the sensor pack
    let sensor_pack = SensorPackService::get_sensor_pack_by_id(&pool, &sensor_pack_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        SensorPackResponseDto::from(sensor_pack),
        "Sensor pack retrieved successfully",
    )))
}

pub async fn get_all_sensor_packs(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Get all sensor packs
    let sensor_packs = SensorPackService::get_all_sensor_packs(&pool).await?;

    // Convert to response DTOs
    let sensor_pack_dtos: Vec<SensorPackResponseDto> = sensor_packs
        .into_iter()
        .map(SensorPackResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_pack_dtos,
        "Sensor packs retrieved successfully",
    )))
}

pub async fn update_sensor_pack(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    sensor_pack_dto: web::Json<UpdateSensorPackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();

    // Validate the sensor pack DTO
    sensor_pack_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can update sensor packs".to_string()));
    }

    // Update the sensor pack
    let sensor_pack = SensorPackService::update_sensor_pack(&pool, &sensor_pack_id, sensor_pack_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        SensorPackResponseDto::from(sensor_pack),
        "Sensor pack updated successfully",
    )))
}

pub async fn delete_sensor_pack(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete sensor packs".to_string()));
    }

    // Delete the sensor pack
    SensorPackService::delete_sensor_pack(&pool, &sensor_pack_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Sensor pack deleted successfully",
    )))
}

