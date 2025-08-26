use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::participant::{CreateParticipantDto, ParticipantResponseDto, UpdateParticipantDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::ParticipantService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn add_participant(
    pool: web::Data<PgPool>,
    participant_dto: web::Json<CreateParticipantDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the participant DTO
    participant_dto.validate()?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the discussion_id is provided
    let discussion_id = match participant_dto.discussion_id {
        Some(id) => id,
        None => return Err(AppError::BadRequest("Discussion ID is required".to_string())),
    };

    // Check if the participant_id is provided
    let participant_id = match participant_dto.participant_id {
        Some(id) => id,
        None => return Err(AppError::BadRequest("Participant ID is required".to_string())),
    };

    // Check if the user is already a participant in the discussion
    let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;
    
    // Check if the user is a participant or has admin role
    if claims.role != UserRole::Admin {
        let is_participant = participants.iter().any(|p| p.participant_id == Some(user_id));
        
        if !is_participant {
            return Err(AppError::Forbidden(
                "You are not a participant in this discussion".to_string(),
            ));
        }
    }

    // Check if the user to be added is already a participant
    let is_already_participant = participants.iter().any(|p| p.participant_id == Some(participant_id));
    
    if is_already_participant {
        return Err(AppError::BadRequest(
            "User is already a participant in this discussion".to_string(),
        ));
    }

    // Add the participant
    let participant = ParticipantService::create_participant(&pool, participant_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        ParticipantResponseDto::from(participant),
        "Participant added successfully",
    )))
}

pub async fn get_participants_by_discussion(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();

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

    // Get the participants
    let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;

    // Convert to response DTOs
    let participant_dtos: Vec<ParticipantResponseDto> = participants
        .into_iter()
        .map(ParticipantResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        participant_dtos,
        "Participants retrieved successfully",
    )))
}

pub async fn remove_participant(
    pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let (discussion_id, participant_id) = path.into_inner();

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user is a participant or has admin role
    if claims.role != UserRole::Admin && user_id != participant_id {
        let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;
        let is_participant = participants.iter().any(|p| p.participant_id == Some(user_id));
        
        if !is_participant {
            return Err(AppError::Forbidden(
                "You are not a participant in this discussion".to_string(),
            ));
        }
    }

    // Remove the participant
    ParticipantService::delete_participant_by_user_and_discussion(&pool, participant_id, discussion_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Participant removed successfully",
    )))
}

