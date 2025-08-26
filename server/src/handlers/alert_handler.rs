use crate::dto::{AlertDto, ApiResponse, CreateAlertDto, UpdateAlertDto};
use crate::errors::AppError;
use crate::models::types::{AlertType, LevelType};
use crate::services::{AlertService, Claims, GroundService, StateService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_alert(
    alert_service: web::Data<Arc<AlertService>>,
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    alert_dto: web::Json<CreateAlertDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    let dto = alert_dto.into_inner();
    
    // Only admins, suppliers, and ground owners can create alerts
    if claims.role != "admin" && claims.role != "supplier" && dto.state_id.is_some() {
        // Check if the user owns the ground with this state's sensor pack
        let state = state_service.get_state_by_id(dto.state_id.unwrap()).await?;
        if let Some(pack_id) = &state.pack_id {
            let grounds = ground_service.get_grounds_by_sensor_pack(pack_id).await?;
            let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
            
            if !is_owner {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let alert = alert_service.create_alert(dto).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        alert,
        "Alert created successfully",
    )))
}

#[get("")]
pub async fn get_all_alerts(
    service: web::Data<Arc<AlertService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can see all alerts
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let alerts = service.get_all_alerts().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alerts,
        "Alerts retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_alert_by_id(
    alert_service: web::Data<Arc<AlertService>>,
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let alert_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    let alert = alert_service.get_alert_by_id(alert_id).await?;
    
    // Check if the user owns the ground with this alert's state
    if claims.role != "admin" && claims.role != "supplier" && alert.state_id.is_some() {
        let state = state_service.get_state_by_id(alert.state_id.unwrap()).await?;
        if let Some(pack_id) = &state.pack_id {
            let grounds = ground_service.get_grounds_by_sensor_pack(pack_id).await?;
            let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
            
            if !is_owner {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alert,
        "Alert retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_alert(
    alert_service: web::Data<Arc<AlertService>>,
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    alert_dto: web::Json<UpdateAlertDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let alert_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current alert to check ownership
    let current_alert = alert_service.get_alert_by_id(alert_id).await?;
    
    // Only admins and suppliers can update alerts
    if claims.role != "admin" && claims.role != "supplier" && current_alert.state_id.is_some() {
        // Check if the user owns the ground with this alert's state
        let state = state_service.get_state_by_id(current_alert.state_id.unwrap()).await?;
        if let Some(pack_id) = &state.pack_id {
            let grounds = ground_service.get_grounds_by_sensor_pack(pack_id).await?;
            let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
            
            if !is_owner {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let alert = alert_service
        .update_alert(alert_id, alert_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alert,
        "Alert updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_alert(
    alert_service: web::Data<Arc<AlertService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let alert_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can delete alerts
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    alert_service.delete_alert(alert_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Alert deleted successfully",
    )))
}

#[get("/state/{state_id}")]
pub async fn get_alerts_by_state_id(
    alert_service: web::Data<Arc<AlertService>>,
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let state_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Check if the user owns the ground with this state
    if claims.role != "admin" && claims.role != "supplier" {
        let state = state_service.get_state_by_id(state_id).await?;
        if let Some(pack_id) = &state.pack_id {
            let grounds = ground_service.get_grounds_by_sensor_pack(pack_id).await?;
            let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
            
            if !is_owner {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let alerts = alert_service.get_alerts_by_state_id(state_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alerts,
        "Alerts retrieved successfully",
    )))
}

#[get("/type/{type}")]
pub async fn get_alerts_by_type(
    service: web::Data<Arc<AlertService>>,
    path: web::Path<AlertType>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let alert_type = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can filter alerts by type
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let alerts = service.get_alerts_by_type(alert_type).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alerts,
        "Alerts retrieved successfully",
    )))
}

#[get("/level/{level}")]
pub async fn get_alerts_by_level(
    service: web::Data<Arc<AlertService>>,
    path: web::Path<LevelType>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let level = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can filter alerts by level
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let alerts = service.get_alerts_by_level(level).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alerts,
        "Alerts retrieved successfully",
    )))
}

#[get("/unseen")]
pub async fn get_unseen_alerts(
    service: web::Data<Arc<AlertService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Only admins can see all unseen alerts
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }
    
    let alerts = service.get_unseen_alerts().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alerts,
        "Unseen alerts retrieved successfully",
    )))
}

#[put("/{id}/mark-seen")]
pub async fn mark_alert_as_seen(
    alert_service: web::Data<Arc<AlertService>>,
    state_service: web::Data<Arc<StateService>>,
    ground_service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let alert_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();
    
    // Get the current alert to check ownership
    let current_alert = alert_service.get_alert_by_id(alert_id).await?;
    
    // Check if the user owns the ground with this alert's state
    if claims.role != "admin" && claims.role != "supplier" && current_alert.state_id.is_some() {
        let state = state_service.get_state_by_id(current_alert.state_id.unwrap()).await?;
        if let Some(pack_id) = &state.pack_id {
            let grounds = ground_service.get_grounds_by_sensor_pack(pack_id).await?;
            let is_owner = grounds.iter().any(|g| g.user_id == Some(claims.user_id));
            
            if !is_owner {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        } else {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }
    }
    
    let alert = alert_service.mark_alert_as_seen(alert_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        alert,
        "Alert marked as seen",
    )))
}

