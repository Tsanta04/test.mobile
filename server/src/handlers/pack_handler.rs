use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::pack::{CreatePackDto, PackResponseDto, UpdatePackDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::PackService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_pack(
    pool: web::Data<PgPool>,
    pack_dto: web::Json<CreatePackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the pack DTO
    pack_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can create packs".to_string()));
    }

    // Create the pack
    let pack = PackService::create_pack(&pool, pack_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        PackResponseDto::from(pack),
        "Pack created successfully",
    )))
}

pub async fn get_pack_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();

    // Get the pack
    let pack = PackService::get_pack_by_id(&pool, pack_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        PackResponseDto::from(pack),
        "Pack retrieved successfully",
    )))
}

pub async fn get_packs_by_pack_id(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();

    // Get the packs
    let packs = PackService::get_packs_by_pack_id(&pool, &pack_id).await?;

    // Convert to response DTOs
    let pack_dtos: Vec<PackResponseDto> = packs
        .into_iter()
        .map(PackResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        pack_dtos,
        "Packs retrieved successfully",
    )))
}

pub async fn get_packs_by_sensor_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();

    // Get the packs
    let packs = PackService::get_packs_by_sensor_id(&pool, sensor_id).await?;

    // Convert to response DTOs
    let pack_dtos: Vec<PackResponseDto> = packs
        .into_iter()
        .map(PackResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        pack_dtos,
        "Packs retrieved successfully",
    )))
}

pub async fn get_all_packs(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Get all packs
    let packs = PackService::get_all_packs(&pool).await?;

    // Convert to response DTOs
    let pack_dtos: Vec<PackResponseDto> = packs
        .into_iter()
        .map(PackResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        pack_dtos,
        "Packs retrieved successfully",
    )))
}

pub async fn update_pack(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    pack_dto: web::Json<UpdatePackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();

    // Validate the pack DTO
    pack_dto.validate()?;

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can update packs".to_string()));
    }

    // Update the pack
    let pack = PackService::update_pack(&pool, pack_id, pack_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        PackResponseDto::from(pack),
        "Pack updated successfully",
    )))
}

pub async fn delete_pack(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();

    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete packs".to_string()));
    }

    // Delete the pack
    PackService::delete_pack(&pool, pack_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Pack deleted successfully",
    )))
}

