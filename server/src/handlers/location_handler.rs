use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::location::{CreateLocationDto, LocationResponseDto, UpdateLocationDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::LocationService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_location(
    pool: web::Data<PgPool>,
    location_dto: web::Json<CreateLocationDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the location DTO
    location_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can create locations".to_string()));
    }

    // Create the location
    let location = LocationService::create_location(&pool, location_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        LocationResponseDto::from(location),
        "Location created successfully",
    )))
}

pub async fn get_location_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let location_id = path.into_inner();

    // Get the location
    let location = LocationService::get_location_by_id(&pool, location_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        LocationResponseDto::from(location),
        "Location retrieved successfully",
    )))
}

pub async fn get_all_locations(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Get all locations
    let locations = LocationService::get_all_locations(&pool).await?;

    // Convert to response DTOs
    let location_dtos: Vec<LocationResponseDto> = locations
        .into_iter()
        .map(LocationResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        location_dtos,
        "Locations retrieved successfully",
    )))
}

pub async fn update_location(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    location_dto: web::Json<UpdateLocationDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let location_id = path.into_inner();

    // Validate the location DTO
    location_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can update locations".to_string()));
    }

    // Update the location
    let location = LocationService::update_location(&pool, location_id, location_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        LocationResponseDto::from(location),
        "Location updated successfully",
    )))
}

pub async fn delete_location(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let location_id = path.into_inner();

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete locations".to_string()));
    }

    // Delete the location
    LocationService::delete_location(&pool, location_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Location deleted successfully",
    )))
}

