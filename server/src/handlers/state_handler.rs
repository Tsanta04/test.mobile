use crate::dto::{ApiResponse, CreateStateDto, StateDto, UpdateStateDto};
use crate::errors::AppError;
use crate::services::{Claims, GroundService, StateService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_state(
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    state_dto: web::Json<CreateStateDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    let dto = state_dto.into_inner();
    
    // Check if the user owns the ground with this sensor pack
    if claims.role != "admin" && claims.role != "supplier" {
        let grounds = ground_service.get_grounds_by_sensor_pack(&dto.pack_id.clone().unwrap_or_default()).await?;
        let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
        
        if !is_owner {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let state = state_service.create_state(dto).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        state,
        "State created successfully",
    )))
}

#[get("")]
pub async fn get_all_states(
    service: web::Data<Arc<StateService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can see all states
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let states = service.get_all_states().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        states,
        "States retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_state_by_id(
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let state_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    let state = state_service.get_state_by_id(state_id).await?;
    
    // Check if the user owns the ground with this sensor pack
    if claims.role != "admin" && claims.role != "supplier" && state.pack_id.is_some() {
        let grounds = ground_service.get_grounds_by_sensor_pack(&state.pack_id.clone().unwrap()).await?;
        let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
        
        if !is_owner {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        state,
        "State retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_state(
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    state_dto: web::Json<UpdateStateDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let state_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current state to check ownership
    let current_state = state_service.get_state_by_id(state_id).await?;
    
    // Only admins and suppliers can update states
    if claims.role != "admin" && claims.role != "supplier" {
        // Check if the user owns the ground with this sensor pack
        if let Some(pack_id) = &current_state.pack_id {
            let grounds = ground_service.get_grounds_by_sensor_pack(pack_id).await?;
            let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
            
            if !is_owner {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let state = state_service
        .update_state(state_id, state_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        state,
        "State updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_state(
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let state_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current state to check ownership
    let current_state = state_service.get_state_by_id(state_id).await?;
    
    // Only admins can delete states
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    state_service.delete_state(state_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "State deleted successfully",
    )))
}

#[get("/pack/{pack_id}")]
pub async fn get_states_by_pack_id(
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if the user owns the ground with this sensor pack
    if claims.role != "admin" && claims.role != "supplier" {
        let grounds = ground_service.get_grounds_by_sensor_pack(&pack_id).await?;
        let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
        
        if !is_owner {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let states = state_service.get_states_by_pack_id(&pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        states,
        "States retrieved successfully",
    )))
}

#[get("/pack/{pack_id}/latest")]
pub async fn get_latest_state_by_pack_id(
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if the user owns the ground with this sensor pack
    if claims.role != "admin" && claims.role != "supplier" {
        let grounds = ground_service.get_grounds_by_sensor_pack(&pack_id).await?;
        let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
        
        if !is_owner {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let state = state_service.get_latest_state_by_pack_id(&pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        state,
        "Latest state retrieved successfully",
    )))
}

