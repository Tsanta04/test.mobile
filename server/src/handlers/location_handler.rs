use crate::dto::{ApiResponse, CreateLocationDto, LocationDto, UpdateLocationDto};
use crate::errors::AppError;
use crate::services::{Claims, LocationService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_location(
    service: web::Data<Arc<LocationService>>,
    location_dto: web::Json<CreateLocationDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and sellers can create locations
    if claims.role != "admin" && claims.role != "seller" {
        return Err(AppError::Forbidden("Admin or seller access required".to_string()));
    }

    let location = service.create_location(location_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        location,
        "Location created successfully",
    )))
}

#[get("")]
pub async fn get_all_locations(
    service: web::Data<Arc<LocationService>>,
) -> Result<HttpResponse, AppError> {
    let locations = service.get_all_locations().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        locations,
        "Locations retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_location_by_id(
    service: web::Data<Arc<LocationService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let location_id = path.into_inner();
    let location = service.get_location_by_id(location_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        location,
        "Location retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_location(
    service: web::Data<Arc<LocationService>>,
    path: web::Path<i32>,
    location_dto: web::Json<UpdateLocationDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let location_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and sellers can update locations
    if claims.role != "admin" && claims.role != "seller" {
        return Err(AppError::Forbidden("Admin or seller access required".to_string()));
    }

    let location = service
        .update_location(location_id, location_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        location,
        "Location updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_location(
    service: web::Data<Arc<LocationService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let location_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can delete locations
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    service.delete_location(location_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Location deleted successfully",
    )))
}

