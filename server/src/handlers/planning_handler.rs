use crate::dto::{ApiResponse, CreatePlanningDto, PlanningDto, UpdatePlanningDto};
use crate::errors::AppError;
use crate::services::{Claims, GroundService, PlanningService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_planning(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    planning_dto: web::Json<CreatePlanningDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    let dto = planning_dto.into_inner();
    
    // Check if the user owns the ground
    if claims.role != "admin" && claims.role != "seller" && dto.ground.is_some() {
        let ground = ground_service.get_ground_by_id(dto.ground.unwrap()).await?;
        if ground.user_id != Some(claims.user_id) {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let planning = planning_service.create_planning(dto).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        planning,
        "Planning created successfully",
    )))
}

#[get("")]
pub async fn get_all_plannings(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can see all plannings
    if claims.role != "admin" {
        // For non-admins, return only plannings for their grounds
        let user_grounds = ground_service.get_grounds_by_user_id(claims.user_id).await?;
        let ground_ids: Vec<i32> = user_grounds.iter().map(|g| g.id).collect();
        
        let all_plannings = planning_service.get_all_plannings().await?;
        let filtered_plannings: Vec<PlanningDto> = all_plannings
            .into_iter()
            .filter(|p| p.ground.map_or(false, |g| ground_ids.contains(&g)))
            .collect();
        
        return Ok(HttpResponse::Ok().json(ApiResponse::success(
            filtered_plannings,
            "Plannings retrieved successfully",
        )));
    }
    
    let plannings = planning_service.get_all_plannings().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        plannings,
        "Plannings retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_planning_by_id(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let planning_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    let planning = planning_service.get_planning_by_id(planning_id).await?;
    
    // Check if the user owns the ground for this planning
    if claims.role != "admin" && claims.role != "seller" && planning.ground.is_some() {
        let ground = ground_service.get_ground_by_id(planning.ground.unwrap()).await?;
        if ground.user_id != Some(claims.user_id) {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        planning,
        "Planning retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_planning(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    planning_dto: web::Json<UpdatePlanningDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let planning_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current planning to check ownership
    let current_planning = planning_service.get_planning_by_id(planning_id).await?;
    
    // Check if the user owns the ground for this planning
    if claims.role != "admin" && claims.role != "seller" && current_planning.ground.is_some() {
        let ground = ground_service.get_ground_by_id(current_planning.ground.unwrap()).await?;
        if ground.user_id != Some(claims.user_id) {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    // If changing the ground, check if user owns the new ground
    let dto = planning_dto.into_inner();
    if dto.ground.is_some() && dto.ground != current_planning.ground && claims.role != "admin" {
        let ground = ground_service.get_ground_by_id(dto.ground.unwrap()).await?;
        if ground.user_id != Some(claims.user_id) {
            return Err(AppError::Forbidden("Access denied for the new ground".to_string()));
        }
    }
    
    let planning = planning_service
        .update_planning(planning_id, dto)
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        planning,
        "Planning updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_planning(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let planning_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current planning to check ownership
    let current_planning = planning_service.get_planning_by_id(planning_id).await?;
    
    // Check if the user owns the ground for this planning
    if claims.role != "admin" && current_planning.ground.is_some() {
        let ground = ground_service.get_ground_by_id(current_planning.ground.unwrap()).await?;
        if ground.user_id != Some(claims.user_id) {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    planning_service.delete_planning(planning_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Planning deleted successfully",
    )))
}

#[get("/ground/{ground_id}")]
pub async fn get_plannings_by_ground_id(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ground_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if the user owns the ground
    if claims.role != "admin" && claims.role != "seller" {
        let ground = ground_service.get_ground_by_id(ground_id).await?;
        if ground.user_id != Some(claims.user_id) {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let plannings = planning_service.get_plannings_by_ground_id(ground_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        plannings,
        "Plannings retrieved successfully",
    )))
}

#[get("/current")]
pub async fn get_current_plannings(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    let current_plannings = planning_service.get_current_plannings().await?;
    
    // Filter plannings for non-admin users
    if claims.role != "admin" && claims.role != "seller" {
        let user_grounds = ground_service.get_grounds_by_user_id(claims.user_id).await?;
        let ground_ids: Vec<i32> = user_grounds.iter().map(|g| g.id).collect();
        
        let filtered_plannings: Vec<PlanningDto> = current_plannings
            .into_iter()
            .filter(|p| p.ground.map_or(false, |g| ground_ids.contains(&g)))
            .collect();
        
        return Ok(HttpResponse::Ok().json(ApiResponse::success(
            filtered_plannings,
            "Current plannings retrieved successfully",
        )));
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        current_plannings,
        "Current plannings retrieved successfully",
    )))
}

#[get("/upcoming")]
pub async fn get_upcoming_plannings(
    planning_service: web::Data<Arc<PlanningService>>,
    ground_service: web::Data<Arc<GroundService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    let upcoming_plannings = planning_service.get_upcoming_plannings().await?;
    
    // Filter plannings for non-admin users
    if claims.role != "admin" && claims.role != "seller" {
        let user_grounds = ground_service.get_grounds_by_user_id(claims.user_id).await?;
        let ground_ids: Vec<i32> = user_grounds.iter().map(|g| g.id).collect();
        
        let filtered_plannings: Vec<PlanningDto> = upcoming_plannings
            .into_iter()
            .filter(|p| p.ground.map_or(false, |g| ground_ids.contains(&g)))
            .collect();
        
        return Ok(HttpResponse::Ok().json(ApiResponse::success(
            filtered_plannings,
            "Upcoming plannings retrieved successfully",
        )));
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        upcoming_plannings,
        "Upcoming plannings retrieved successfully",
    )))
}

