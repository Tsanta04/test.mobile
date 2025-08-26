use crate::dto::{ApiResponse, CreateSensorTypeDto, SensorTypeDto, UpdateSensorTypeDto};
use crate::errors::AppError;
use crate::services::{Claims, SensorTypeService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_sensor_type(
    service: web::Data<Arc<SensorTypeService>>,
    sensor_type_dto: web::Json<CreateSensorTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can create sensor types
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let sensor_type = service.create_sensor_type(sensor_type_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        sensor_type,
        "Sensor type created successfully",
    )))
}

#[get("")]
pub async fn get_all_sensor_types(
    service: web::Data<Arc<SensorTypeService>>,
) -> Result<HttpResponse, AppError> {
    let sensor_types = service.get_all_sensor_types().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_types,
        "Sensor types retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_sensor_type_by_id(
    service: web::Data<Arc<SensorTypeService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sensor_type_id = path.into_inner();
    let sensor_type = service.get_sensor_type_by_id(sensor_type_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_type,
        "Sensor type retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_sensor_type(
    service: web::Data<Arc<SensorTypeService>>,
    path: web::Path<i32>,
    sensor_type_dto: web::Json<UpdateSensorTypeDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_type_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can update sensor types
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let sensor_type = service
        .update_sensor_type(sensor_type_id, sensor_type_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor_type,
        "Sensor type updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_sensor_type(
    service: web::Data<Arc<SensorTypeService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_type_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can delete sensor types
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    service.delete_sensor_type(sensor_type_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Sensor type deleted successfully",
    )))
}

