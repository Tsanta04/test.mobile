use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::message::{CreateMessageDto, MessageResponseDto, UpdateMessageDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::{MessageService, ParticipantService};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_message(
    pool: web::Data<PgPool>,
    message_dto: web::Json<CreateMessageDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the message DTO
    message_dto.validate()?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the discussion_id is provided
    let discussion_id = match message_dto.discussion_id {
        Some(id) => id,
        None => return Err(AppError::BadRequest("Discussion ID is required".to_string())),
    };

    // Check if the user is a participant in the discussion
    if claims.role != UserRole::Admin {
        let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;
        let is_participant = participants.iter().any(|p| p.participant_id == Some(user_id));
        
        if !is_participant {
            return Err(AppError::Forbidden(
                "You are not a participant in this discussion".to_string(),
            ));
        }
    }

    // Create a new message DTO with the sender_id set to the current user
    let mut new_message_dto = message_dto.0;
    new_message_dto.sender_id = Some(user_id);

    // Create the message
    let message = MessageService::create_message(&pool, new_message_dto).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        MessageResponseDto::from(message),
        "Message sent successfully",
    )))
}

pub async fn get_message_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();

    // Get the message
    let message = MessageService::get_message_by_id(&pool, message_id).await?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user is a participant in the discussion or has admin role
    if claims.role != UserRole::Admin {
        if let Some(discussion_id) = message.discussion_id {
            let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;
            let is_participant = participants.iter().any(|p| p.participant_id == Some(user_id));
            
            if !is_participant {
                return Err(AppError::Forbidden(
                    "You are not a participant in this discussion".to_string(),
                ));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        MessageResponseDto::from(message),
        "Message retrieved successfully",
    )))
}

pub async fn get_messages_by_discussion(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user is a participant in the discussion or has admin role
    if claims.role != UserRole::Admin {
        let participants = ParticipantService::get_participants_by_discussion(&pool, discussion_id).await?;
        let is_participant = participants.iter().any(|p| p.participant_id == Some(user_id));
        
        if !is_participant {
            return Err(AppError::Forbidden(
                "You are not a participant in this discussion".to_string(),
            ));
        }
    }

    // Get the messages
    let messages = MessageService::get_messages_by_discussion(&pool, discussion_id).await?;

    // Convert to response DTOs
    let message_dtos: Vec<MessageResponseDto> = messages
        .into_iter()
        .map(MessageResponseDto::from)
        .collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        message_dtos,
        "Messages retrieved successfully",
    )))
}

pub async fn update_message(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    message_dto: web::Json<UpdateMessageDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();

    // Validate the message DTO
    message_dto.validate()?;

    // Get the message
    let message = MessageService::get_message_by_id(&pool, message_id).await?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user is the sender of the message or has admin role
    if message.sender_id != Some(user_id) && claims.role != UserRole::Admin {
        return Err(AppError::Forbidden(
            "You can only update your own messages".to_string(),
        ));
    }

    // Update the message
    let updated_message = MessageService::update_message(&pool, message_id, message_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        MessageResponseDto::from(updated_message),
        "Message updated successfully",
    )))
}

pub async fn delete_message(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();

    // Get the message
    let message = MessageService::get_message_by_id(&pool, message_id).await?;

    // Get the user ID from the claims
    let claims = req.claims()?;
    let user_id = claims.user_id;

    // Check if the user is the sender of the message or has admin role
    if message.sender_id != Some(user_id) && claims.role != UserRole::Admin {
        return Err(AppError::Forbidden(
            "You can only delete your own messages".to_string(),
        ));
    }

    // Delete the message
    MessageService::delete_message(&pool, message_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Message deleted successfully",
    )))
}

