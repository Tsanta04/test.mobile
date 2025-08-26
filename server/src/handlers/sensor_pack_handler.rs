use crate::dto::{ApiResponse, CreateSensorPackDto, SensorPackDto, UpdateSensorPackDto};
use crate::errors::AppError;
use crate::services::{Claims, SensorPackService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_sensor_pack(
    service: web::Data<Arc<SensorPackService>>,
    sensor_pack_dto: web::Json<CreateSensorPackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can create sensor packs
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let sensor_pack = service.create_sensor_pack(sensor_pack_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        sensor_pack,
        "Sensor pack created successfully",
    )))
}

#[get("")]
pub async fn get_all_sensor_packs(
    service: web::Data<Arc<SensorPackService>>,
) -> Result<HttpResponse, AppError> {
    let sensor_packs = service.get_all_sensor_packs().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_packs,
        "Sensor packs retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_sensor_pack_by_id(
    service: web::Data<Arc<SensorPackService>>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();
    let sensor_pack = service.get_sensor_pack_by_id(&sensor_pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_pack,
        "Sensor pack retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_sensor_pack(
    service: web::Data<Arc<SensorPackService>>,
    path: web::Path<String>,
    sensor_pack_dto: web::Json<UpdateSensorPackDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can update sensor packs
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let sensor_pack = service
        .update_sensor_pack(&sensor_pack_id, sensor_pack_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_pack,
        "Sensor pack updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_sensor_pack(
    service: web::Data<Arc<SensorPackService>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_pack_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can delete sensor packs
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    service.delete_sensor_pack(&sensor_pack_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Sensor pack deleted successfully",
    )))
}

