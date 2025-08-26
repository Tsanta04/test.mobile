use crate::dto::{ApiResponse, CreateMessageDto, MessageDto, UpdateMessageDto};
use crate::errors::AppError;
use crate::services::{Claims, MessageService, ParticipantService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_message(
    message_service: web::Data<Arc<MessageService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    message_dto: web::Json<CreateMessageDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    let mut dto = message_dto.into_inner();
    
    // If sender_id is not provided, use the current user's ID
    if dto.sender_id.is_none() {
        dto.sender_id = Some(claims.user_id);
    }
    
    // Users can only send messages as themselves unless they are admins
    if dto.sender_id.unwrap() != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }
    
    // Check if user is a participant in the discussion
    if let Some(discussion_id) = dto.discussion_id {
        let participations = participant_service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == Some(discussion_id));
        
        if !is_participant && claims.role != "admin" {
            return Err(AppError::Forbidden("You are not a participant in this discussion".to_string()));
        }
    }
    
    let message = message_service.create_message(dto).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        message,
        "Message sent successfully",
    )))
}

#[get("")]
pub async fn get_all_messages(
    service: web::Data<Arc<MessageService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can see all messages
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let messages = service.get_all_messages().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        messages,
        "Messages retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_message_by_id(
    message_service: web::Data<Arc<MessageService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    let message = message_service.get_message_by_id(message_id).await?;
    
    // Check if user is a participant in the discussion or admin
    if claims.role != "admin" && message.discussion_id.is_some() {
        let participations = participant_service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == message.discussion_id);
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        message,
        "Message retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_message(
    message_service: web::Data<Arc<MessageService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    message_dto: web::Json<UpdateMessageDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current message to check ownership
    let current_message = message_service.get_message_by_id(message_id).await?;
    
    // Users can only update their own messages unless they are admins
    if current_message.sender_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }
    
    // Check if user is a participant in the discussion
    if let Some(discussion_id) = current_message.discussion_id {
        let participations = participant_service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == Some(discussion_id));
        
        if !is_participant && claims.role != "admin" {
            return Err(AppError::Forbidden("You are not a participant in this discussion".to_string()));
        }
    }
    
    let message = message_service
        .update_message(message_id, message_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        message,
        "Message updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_message(
    message_service: web::Data<Arc<MessageService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let message_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current message to check ownership
    let current_message = message_service.get_message_by_id(message_id).await?;
    
    // Users can only delete their own messages unless they are admins
    if current_message.sender_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }
    
    message_service.delete_message(message_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Message deleted successfully",
    )))
}

#[get("/discussion/{discussion_id}")]
pub async fn get_messages_by_discussion_id(
    message_service: web::Data<Arc<MessageService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if user is a participant in the discussion or admin
    if claims.role != "admin" {
        let participations = participant_service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == Some(discussion_id));
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let messages = message_service.get_messages_by_discussion_id(discussion_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        messages,
        "Messages retrieved successfully",
    )))
}

