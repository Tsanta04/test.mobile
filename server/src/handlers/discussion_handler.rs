use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::discussion::{CreateDiscussionDto, DiscussionResponseDto, UpdateDiscussionDto};
use crate::dto::participant::CreateParticipantDto;
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::{DiscussionService, ParticipantService};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_discussion(
    pool: web::Data<PgPool>,
    discussion_dto: web::Json<CreateDiscussionDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the discussion DTO
    discussion_dto.validate()?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Start a transaction
    let mut tx = pool.begin().await.map_err(|e| {
        AppError::Database(format!("Failed to start transaction: {}", e))
    })?;

    // Create the discussion
    let discussion = DiscussionService::create_discussion(&pool, discussion_dto.0).await?;

    // Add the creator as a participant
    let participant_dto = CreateParticipantDto {
        discussion_id: Some(discussion.id),
        participant_id: Some(user_id),
    };
    ParticipantService::create_participant(&pool, participant_dto).await?;

    // Commit the transaction
    tx.commit().await.map_err(|e| {
        AppError::Database(format!("Failed to commit transaction: {}", e))
    })?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        DiscussionResponseDto::from(discussion),
        "Discussion created successfully",
    )))
}

pub async fn get_discussion_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Get the discussion
    let discussion = DiscussionService::get_discussion_by_id(&pool, discussion_id).await?;

    // Check if the user is a participant or has admin role
    if claims.role != UserRole::Admin {
        let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;
        let is_participant = participants.iter().any(|p| p.participant_id == Some(user_id));
        
        if !is_participant {
            return Err(AppError::Forbidden(
                "You are not a participant in this discussion".to_string(),
            ));
        }
    }

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        DiscussionResponseDto::from(discussion),
        "Discussion retrieved successfully",
    )))
}

pub async fn get_discussions_by_participant(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Get the discussions
    let discussions = DiscussionService::get_discussions_by_participant(&pool, user_id).await?;

    // Convert to response DTOs
    let discussion_dtos: Vec<DiscussionResponseDto> = discussions
        .into_iter()
        .map(DiscussionResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        discussion_dtos,
        "Discussions retrieved successfully",
    )))
}

pub async fn get_all_discussions(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can view all discussions".to_string()));
    }

    // Get all discussions
    let discussions = DiscussionService::get_all_discussions(&pool).await?;

    // Convert to response DTOs
    let discussion_dtos: Vec<DiscussionResponseDto> = discussions
        .into_iter()
        .map(DiscussionResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        discussion_dtos,
        "Discussions retrieved successfully",
    )))
}

pub async fn update_discussion(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    discussion_dto: web::Json<UpdateDiscussionDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();

    // Validate the discussion DTO
    discussion_dto.validate()?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user is a participant or has admin role
    if claims.role != UserRole::Admin {
        let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;
        let is_participant = participants.iter().any(|p| p.participant_id == Some(user_id));
        
        if !is_participant {
            return Err(AppError::Forbidden(
                "You are not a participant in this discussion".to_string(),
            ));
        }
    }

    // Update the discussion
    let discussion = DiscussionService::update_discussion(&pool, discussion_id, discussion_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        DiscussionResponseDto::from(discussion),
        "Discussion updated successfully",
    )))
}

pub async fn delete_discussion(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();

    // Get the user ID from the claims
    let claims = req.claims()?;
    
    // Only admins can delete discussions
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete discussions".to_string()));
    }

    // Delete the discussion
    DiscussionService::delete_discussion(&pool, discussion_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Discussion deleted successfully",
    )))
}

