use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::planning::{CreatePlanningDto, PlanningResponseDto, UpdatePlanningDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::{GroundService, PlanningService};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_planning(
    pool: web::Data<PgPool>,
    planning_dto: web::Json<CreatePlanningDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the planning DTO
    planning_dto.validate()?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the ground_id is provided
    let ground_id = match planning_dto.ground {
        Some(id) => id,
        None => return Err(AppError::BadRequest("Ground ID is required".to_string())),
    };

    // Check if the user owns the ground or has admin role
    if claims.role != UserRole::Admin {
        let ground = GroundService::get_ground_by_id(&pool, ground_id).await?;
        if ground.user_id != Some(user_id) {
            return Err(AppError::Forbidden(
                "You do not have permission to create planning for this ground".to_string(),
            ));
        }
    }

    // Create the planning
    let planning = PlanningService::create_planning(&pool, planning_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        PlanningResponseDto::from(planning),
        "Planning created successfully",
    )))
}

pub async fn get_planning_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let planning_id = path.into_inner();

    // Get the planning
    let planning = PlanningService::get_planning_by_id(&pool, planning_id).await?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user owns the ground associated with this planning or has admin role
    if claims.role != UserRole::Admin {
        if let Some(ground_id) = planning.ground {
            let ground = GroundService::get_ground_by_id(&pool, ground_id).await?;
            if ground.user_id != Some(user_id) {
                return Err(AppError::Forbidden(
                    "You do not have permission to view this planning".to_string(),
                ));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        PlanningResponseDto::from(planning),
        "Planning retrieved successfully",
    )))
}

pub async fn get_plannings_by_ground(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ground_id = path.into_inner();

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user owns the ground or has admin role
    if claims.role != UserRole::Admin {
        let ground = GroundService::get_ground_by_id(&pool, ground_id).await?;
        if ground.user_id != Some(user_id) {
            return Err(AppError::Forbidden(
                "You do not have permission to view plannings for this ground".to_string(),
            ));
        }
    }

    // Get the plannings
    let plannings = PlanningService::get_plannings_by_ground(&pool, ground_id).await?;

    // Convert to response DTOs
    let planning_dtos: Vec<PlanningResponseDto> = plannings
        .into_iter()
        .map(PlanningResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        planning_dtos,
        "Plannings retrieved successfully",
    )))
}

pub async fn get_plannings_by_date_range(
    pool: web::Data<PgPool>,
    query: web::Query<crate::dto::planning::DateRangeQuery>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Parse the date range
    let start_date = chrono::DateTime::parse_from_rfc3339(&query.start_date)
        .map_err(|_| AppError::BadRequest("Invalid start date format".to_string()))?
        .with_timezone(&chrono::Utc);
    
    let end_date = chrono::DateTime::parse_from_rfc3339(&query.end_date)
        .map_err(|_| AppError::BadRequest("Invalid end date format".to_string()))?
        .with_timezone(&chrono::Utc);

    // Get the user ID from the claims
    let claims = req.claims()?;
    
    // Only admins can view all plannings by date range
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can view all plannings by date range".to_string()));
    }

    // Get the plannings
    let plannings = PlanningService::get_plannings_by_date_range(&pool, start_date, end_date).await?;

    // Convert to response DTOs
    let planning_dtos: Vec<PlanningResponseDto> = plannings
        .into_iter()
        .map(PlanningResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        planning_dtos,
        "Plannings retrieved successfully",
    )))
}

pub async fn get_all_plannings(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can view all plannings".to_string()));
    }

    // Get all plannings
    let plannings = PlanningService::get_all_plannings(&pool).await?;

    // Convert to response DTOs
    let planning_dtos: Vec<PlanningResponseDto> = plannings
        .into_iter()
        .map(PlanningResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        planning_dtos,
        "Plannings retrieved successfully",
    )))
}

pub async fn update_planning(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    planning_dto: web::Json<UpdatePlanningDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let planning_id = path.into_inner();

    // Validate the planning DTO
    planning_dto.validate()?;

    // Get the planning
    let planning = PlanningService::get_planning_by_id(&pool, planning_id).await?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user owns the ground associated with this planning or has admin role
    if claims.role != UserRole::Admin {
        if let Some(ground_id) = planning.ground {
            let ground = GroundService::get_ground_by_id(&pool, ground_id).await?;
            if ground.user_id != Some(user_id) {
                return Err(AppError::Forbidden(
                    "You do not have permission to update this planning".to_string(),
                ));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }

    // If the user is trying to change the ground, check if they own the new ground
    if let Some(new_ground_id) = planning_dto.ground {
        if claims.role != UserRole::Admin {
            let new_ground = GroundService::get_ground_by_id(&pool, new_ground_id).await?;
            if new_ground.user_id != Some(user_id) {
                return Err(AppError::Forbidden(
                    "You do not have permission to set this ground".to_string(),
                ));
            }
        }
    }

    // Update the planning
    let updated_planning = PlanningService::update_planning(&pool, planning_id, planning_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        PlanningResponseDto::from(updated_planning),
        "Planning updated successfully",
    )))
}

pub async fn delete_planning(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let planning_id = path.into_inner();

    // Get the planning
    let planning = PlanningService::get_planning_by_id(&pool, planning_id).await?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user owns the ground associated with this planning or has admin role
    if claims.role != UserRole::Admin {
        if let Some(ground_id) = planning.ground {
            let ground = GroundService::get_ground_by_id(&pool, ground_id).await?;
            if ground.user_id != Some(user_id) {
                return Err(AppError::Forbidden(
                    "You do not have permission to delete this planning".to_string(),
                ));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }

    // Delete the planning
    PlanningService::delete_planning(&pool, planning_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Planning deleted successfully",
    )))
}

