use crate::dto::{ApiResponse, CreateDiscussionDto, DiscussionDto, UpdateDiscussionDto};
use crate::errors::AppError;
use crate::services::{Claims, DiscussionService, ParticipantService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_discussion(
    discussion_service: web::Data<Arc<DiscussionService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    discussion_dto: web::Json<CreateDiscussionDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Create the discussion
    let discussion = discussion_service.create_discussion(discussion_dto.into_inner()).await?;
    
    // Add the creator as a participant
    use crate::dto::CreateParticipantDto;
    let participant_dto = CreateParticipantDto {
        discussion_id: Some(discussion.id),
        participant_id: Some(claims.user_id),
    };
    
    participant_service.create_participant(participant_dto).await?;
    
    Ok(HttpResponse::Created().json(ApiResponse::success(
        discussion,
        "Discussion created successfully",
    )))
}

#[get("")]
pub async fn get_all_discussions(
    discussion_service: web::Data<Arc<DiscussionService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can see all discussions
    if claims.role != "admin" {
        // For non-admins, return only discussions they participate in
        let participations = participant_service.get_discussions_by_participant_id(claims.user_id).await?;
        let discussion_ids: Vec<i32> = participations
            .iter()
            .filter_map(|p| p.discussion_id)
            .collect();
        
        let mut discussions = Vec::new();
        for id in discussion_ids {
            if let Ok(discussion) = discussion_service.get_discussion_by_id(id).await {
                discussions.push(discussion);
            }
        }
        
        return Ok(HttpResponse::Ok().json(ApiResponse::success(
            discussions,
            "Discussions retrieved successfully",
        )));
    }
    
    let discussions = discussion_service.get_all_discussions().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        discussions,
        "Discussions retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_discussion_by_id(
    discussion_service: web::Data<Arc<DiscussionService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if user is a participant or admin
    if claims.role != "admin" {
        let participations = participant_service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == Some(discussion_id));
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let discussion = discussion_service.get_discussion_by_id(discussion_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        discussion,
        "Discussion retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_discussion(
    discussion_service: web::Data<Arc<DiscussionService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    discussion_dto: web::Json<UpdateDiscussionDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if user is a participant or admin
    if claims.role != "admin" {
        let participations = participant_service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == Some(discussion_id));
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let discussion = discussion_service
        .update_discussion(discussion_id, discussion_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        discussion,
        "Discussion updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_discussion(
    discussion_service: web::Data<Arc<DiscussionService>>,
    participant_service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can delete discussions
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    discussion_service.delete_discussion(discussion_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Discussion deleted successfully",
    )))
}

