use crate::dto::{ApiResponse, CreateSensorDto, SensorDto, UpdateSensorDto};
use crate::errors::AppError;
use crate::services::{Claims, SensorService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_sensor(
    service: web::Data<Arc<SensorService>>,
    sensor_dto: web::Json<CreateSensorDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can create sensors
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let sensor = service.create_sensor(sensor_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(
        sensor,
        "Sensor created successfully",
    )))
}

#[get("")]
pub async fn get_all_sensors(
    service: web::Data<Arc<SensorService>>,
) -> Result<HttpResponse, AppError> {
    let sensors = service.get_all_sensors().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensors,
        "Sensors retrieved successfully",
    )))
}

#[get("/{id}")]
pub async fn get_sensor_by_id(
    service: web::Data<Arc<SensorService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();
    let sensor = service.get_sensor_by_id(sensor_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor,
        "Sensor retrieved successfully",
    )))
}

#[put("/{id}")]
pub async fn update_sensor(
    service: web::Data<Arc<SensorService>>,
    path: web::Path<i32>,
    sensor_dto: web::Json<UpdateSensorDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins and suppliers can update sensors
    if claims.role != "admin" && claims.role != "supplier" {
        return Err(AppError::Forbidden("Admin or supplier access required".to_string()));
    }

    let sensor = service
        .update_sensor(sensor_id, sensor_dto.into_inner())
        .await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensor,
        "Sensor updated successfully",
    )))
}

#[delete("/{id}")]
pub async fn delete_sensor(
    service: web::Data<Arc<SensorService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let sensor_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can delete sensors
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    service.delete_sensor(sensor_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success(
        (),
        "Sensor deleted successfully",
    )))
}

#[get("/type/{type_id}")]
pub async fn get_sensors_by_type(
    service: web::Data<Arc<SensorService>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let type_id = path.into_inner();
    let sensors = service.get_sensors_by_type(type_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        sensors,
        "Sensors retrieved successfully",
    )))
}

