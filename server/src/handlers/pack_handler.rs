use crate::dto::{ApiResponse, CreatePackDto, PackDto, UpdatePackDto};
use crate::errors::AppError;
use crate::services::{Claims, PackService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_pack(
    service: web::Data<Arc<PackService>>,
    pack_dto: web::Json<CreatePackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can create packs
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let pack = service.create_pack(pack_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        pack,
        "Pack created successfully",
    )))
}

#[get("")]
pub async fn get_all_packs(
    service: web::Data<Arc<PackService>>,
) -> Result<HttpResponse, AppError> {
    let packs = service.get_all_packs().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        packs,
        "Packs retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_pack_by_id(
    service: web::Data<Arc<PackService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();
    let pack = service.get_pack_by_id(pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        pack,
        "Pack retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_pack(
    service: web::Data<Arc<PackService>>,
    path: web::Path<i32>,
    pack_dto: web::Json<UpdatePackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can update packs
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let pack = service
        .update_pack(pack_id, pack_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        pack,
        "Pack updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_pack(
    service: web::Data<Arc<PackService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let pack_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can delete packs
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    service.delete_pack(pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Pack deleted successfully",
    )))
}

#[get("/sensor-pack/{id}")]
pub async fn get_packs_by_sensor_pack_id(
    service: web::Data<Arc<PackService>>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();
    let packs = service.get_packs_by_sensor_pack_id(&sensor_pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        packs,
        "Packs retrieved successfully",
    )))
}

#[get("/sensor/{id}")]
pub async fn get_packs_by_sensor_id(
    service: web::Data<Arc<PackService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();
    let packs = service.get_packs_by_sensor_id(sensor_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        packs,
        "Packs retrieved successfully",
    )))
}

