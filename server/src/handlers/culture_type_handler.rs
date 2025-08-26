use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::culture_type::{CreateCultureTypeDto, CultureTypeResponseDto, UpdateCultureTypeDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::CultureTypeService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_culture_type(
    pool: web::Data<PgPool>,
    culture_type_dto: web::Json<CreateCultureTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the culture type DTO
    culture_type_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can create culture types".to_string()));
    }

    // Create the culture type
    let culture_type = CultureTypeService::create_culture_type(&pool, culture_type_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        CultureTypeResponseDto::from(culture_type),
        "Culture type created successfully",
    )))
}

pub async fn get_culture_type_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let culture_type_id = path.into_inner();

    // Get the culture type
    let culture_type = CultureTypeService::get_culture_type_by_id(&pool, culture_type_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        CultureTypeResponseDto::from(culture_type),
        "Culture type retrieved successfully",
    )))
}

pub async fn get_all_culture_types(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Get all culture types
    let culture_types = CultureTypeService::get_all_culture_types(&pool).await?;

    // Convert to response DTOs
    let culture_type_dtos: Vec<CultureTypeResponseDto> = culture_types
        .into_iter()
        .map(CultureTypeResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        culture_type_dtos,
        "Culture types retrieved successfully",
    )))
}

pub async fn update_culture_type(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    culture_type_dto: web::Json<UpdateCultureTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let culture_type_id = path.into_inner();

    // Validate the culture type DTO
    culture_type_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can update culture types".to_string()));
    }

    // Update the culture type
    let culture_type = CultureTypeService::update_culture_type(&pool, culture_type_id, culture_type_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        CultureTypeResponseDto::from(culture_type),
        "Culture type updated successfully",
    )))
}

pub async fn delete_culture_type(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let culture_type_id = path.into_inner();

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete culture types".to_string()));
    }

    // Delete the culture type
    CultureTypeService::delete_culture_type(&pool, culture_type_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Culture type deleted successfully",
    )))
}

