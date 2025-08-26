use crate::dto::{ApiResponse, CreateGroundDto, GroundDto, UpdateGroundDto};
use crate::errors::AppError;
use crate::services::{Claims, GroundService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_ground(
    service: web::Data<Arc<GroundService>>,
    ground_dto: web::Json<CreateGroundDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    let mut dto = ground_dto.into_inner();

    // If user_id is not provided, use the current user's ID
    if dto.user_id.is_none() {
        dto.user_id = Some(claims.user_id);
    }

    // Only admins can create grounds for other users
    if dto.user_id.unwrap() != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    // Only sellers, admins, and the ground owner can create grounds
    if claims.role != "admin" && claims.role != "seller" && dto.user_id.unwrap() != claims.user_id {
        return Err(AppError::Forbidden("Admin or seller access required".to_string()));
    }

    let ground = service.create_ground(dto).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        ground,
        "Ground created successfully",
    )))
}

#[get("")]
pub async fn get_all_grounds(
    service: web::Data<Arc<GroundService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can see all grounds
    if claims.role != "admin" {
        // For non-admins, return only their grounds
        let grounds = service.get_grounds_by_user_id(claims.user_id).await?;
        return Ok(HttpResponse::Ok().json(ApiResponse::success(
            grounds,
            "Grounds retrieved successfully",
        )));
    }

    let grounds = service.get_all_grounds().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        grounds,
        "Grounds retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_ground_by_id(
    service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ground_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    let ground = service.get_ground_by_id(ground_id).await?;

    // Users can only access their own grounds unless they are admins or sellers
    if ground.user_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" && claims.role != "seller" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(
        ground,
        "Ground retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_ground(
    service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    ground_dto: web::Json<UpdateGroundDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ground_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Get the current ground to check ownership
    let current_ground = service.get_ground_by_id(ground_id).await?;

    // Users can only update their own grounds unless they are admins or sellers
    if current_ground.user_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" && claims.role != "seller" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    // Only admins can change user_id
    let dto = ground_dto.into_inner();
    if dto.user_id.is_some() && dto.user_id.unwrap() != current_ground.user_id.unwrap_or(-1) && claims.role != "admin" {
        return Err(AppError::Forbidden("Only admins can change ground ownership".to_string()));
    }

    let ground = service.update_ground(ground_id, dto).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        ground,
        "Ground updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_ground(
    service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ground_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Get the current ground to check ownership
    let current_ground = service.get_ground_by_id(ground_id).await?;

    // Users can only delete their own grounds unless they are admins
    if current_ground.user_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    service.delete_ground(ground_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Ground deleted successfully",
    )))
}

#[get("/user/{user_id}")]
pub async fn get_grounds_by_user_id(
    service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Users can only access their own grounds unless they are admins or sellers
    if user_id != claims.user_id && claims.role != "admin" && claims.role != "seller" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let grounds = service.get_grounds_by_user_id(user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        grounds,
        "Grounds retrieved successfully",
    )))
}

#[get("/culture-type/{culture_type_id}")]
pub async fn get_grounds_by_culture_type(
    service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let culture_type_id = path.into_inner();
    let grounds = service.get_grounds_by_culture_type(culture_type_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        grounds,
        "Grounds retrieved successfully",
    )))
}

#[get("/location/{location_id}")]
pub async fn get_grounds_by_location(
    service: web::Data<Arc<GroundService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let location_id = path.into_inner();
    let grounds = service.get_grounds_by_location(location_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        grounds,
        "Grounds retrieved successfully",
    )))
}

#[get("/sensor-pack/{sensor_pack_id}")]
pub async fn get_grounds_by_sensor_pack(
    service: web::Data<Arc<GroundService>>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();
    let grounds = service.get_grounds_by_sensor_pack(&sensor_pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        grounds,
        "Grounds retrieved successfully",
    )))
}

