use crate::dto::{ApiResponse, CreateCultureTypeDto, CultureTypeDto, UpdateCultureTypeDto};
use crate::errors::AppError;
use crate::services::{Claims, CultureTypeService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_culture_type(
    service: web::Data<Arc<CultureTypeService>>,
    culture_type_dto: web::Json<CreateCultureTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can create culture types
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let culture_type = service.create_culture_type(culture_type_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        culture_type,
        "Culture type created successfully",
    )))
}

#[get("")]
pub async fn get_all_culture_types(
    service: web::Data<Arc<CultureTypeService>>,
) -> Result<HttpResponse, AppError> {
    let culture_types = service.get_all_culture_types().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        culture_types,
        "Culture types retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_culture_type_by_id(
    service: web::Data<Arc<CultureTypeService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let culture_type_id = path.into_inner();
    let culture_type = service.get_culture_type_by_id(culture_type_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        culture_type,
        "Culture type retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_culture_type(
    service: web::Data<Arc<CultureTypeService>>,
    path: web::Path<i32>,
    culture_type_dto: web::Json<UpdateCultureTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let culture_type_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can update culture types
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let culture_type = service
        .update_culture_type(culture_type_id, culture_type_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        culture_type,
        "Culture type updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_culture_type(
    service: web::Data<Arc<CultureTypeService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let culture_type_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can delete culture types
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    service.delete_culture_type(culture_type_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Culture type deleted successfully",
    )))
}

