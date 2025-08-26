use crate::dto::{ApiResponse, CreateParticipantDto, ParticipantDto, UpdateParticipantDto};
use crate::errors::AppError;
use crate::services::{Claims, ParticipantService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_participant(
    service: web::Data<Arc<ParticipantService>>,
    participant_dto: web::Json<CreateParticipantDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if user is already a participant in the discussion
    if let Some(discussion_id) = participant_dto.discussion_id {
        let participations = service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == Some(discussion_id));
        
        // Only existing participants or admins can add new participants
        if !is_participant && claims.role != "admin" {
            return Err(AppError::Forbidden("You are not a participant in this discussion".to_string()));
        }
    }
    
    let participant = service.create_participant(participant_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        participant,
        "Participant added successfully",
    )))
}

#[get("")]
pub async fn get_all_participants(
    service: web::Data<Arc<ParticipantService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can see all participants
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let participants = service.get_all_participants().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        participants,
        "Participants retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_participant_by_id(
    service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    let participant = service.get_participant_by_id(participant_id).await?;
    
    // Check if user is a participant in the same discussion or admin
    if claims.role != "admin" && participant.discussion_id.is_some() {
        let participations = service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == participant.discussion_id);
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        participant,
        "Participant retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_participant(
    service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    participant_dto: web::Json<UpdateParticipantDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current participant to check discussion
    let current_participant = service.get_participant_by_id(participant_id).await?;
    
    // Check if user is a participant in the same discussion or admin
    if claims.role != "admin" && current_participant.discussion_id.is_some() {
        let participations = service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == current_participant.discussion_id);
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let participant = service
        .update_participant(participant_id, participant_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        participant,
        "Participant updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_participant(
    service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let participant_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current participant to check discussion
    let current_participant = service.get_participant_by_id(participant_id).await?;
    
    // Users can remove themselves, or admins can remove anyone
    let is_self_removal = current_participant.participant_id == Some(claims.user_id);
    
    // Check if user is a participant in the same discussion or admin
    if !is_self_removal && claims.role != "admin" && current_participant.discussion_id.is_some() {
        let participations = service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == current_participant.discussion_id);
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    service.delete_participant(participant_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Participant removed successfully",
    )))
}

#[get("/discussion/{discussion_id}")]
pub async fn get_participants_by_discussion_id(
    service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let discussion_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if user is a participant in the discussion or admin
    if claims.role != "admin" {
        let participations = service.get_discussions_by_participant_id(claims.user_id).await?;
        let is_participant = participations
            .iter()
            .any(|p| p.discussion_id == Some(discussion_id));
        
        if !is_participant {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let participants = service.get_participants_by_discussion_id(discussion_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        participants,
        "Participants retrieved successfully",
    )))
}

#[get("/user/{user_id}")]
pub async fn get_discussions_by_participant_id(
    service: web::Data<Arc<ParticipantService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Users can only access their own discussions unless they are admins
    if user_id != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }
    
    let participations = service.get_discussions_by_participant_id(user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        participations,
        "Discussions retrieved successfully",
    )))
}

