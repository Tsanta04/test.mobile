use crate::dto::{ApiResponse, CreatePersonDto, PersonDto, UpdatePersonDto};
use crate::errors::AppError;
use crate::services::{Claims, PersonService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("")]
pub async fn create_person(
    service: web::Data<Arc<PersonService>>,
    person_dto: web::Json<CreatePersonDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    let mut dto = person_dto.into_inner();

    // If user_id is not provided, use the current user's ID
    if dto.user_id.is_none() {
        dto.user_id = Some(claims.user_id);
    }

    // Only admins can create persons for other users
    if dto.user_id.unwrap() != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let person = service.create_person(dto).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(person, "Person created successfully")))
}

#[get("")]
pub async fn get_all_persons(
    service: web::Data<Arc<PersonService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();

    // Only admins can see all persons
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let persons = service.get_all_persons().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(persons, "Persons retrieved successfully")))
}

#[get("/{id}")]
pub async fn get_person_by_id(
    service: web::Data<Arc<PersonService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let person_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    let person = service.get_person_by_id(person_id).await?;

    // Users can only access their own person data unless they are admins
    if person.user_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::success(person, "Person retrieved successfully")))
}

#[put("/{id}")]
pub async fn update_person(
    service: web::Data<Arc<PersonService>>,
    path: web::Path<i32>,
    person_dto: web::Json<UpdatePersonDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let person_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Get the current person to check ownership
    let current_person = service.get_person_by_id(person_id).await?;

    // Users can only update their own person data unless they are admins
    if current_person.user_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    // Only admins can change user_id
    let dto = person_dto.into_inner();
    if dto.user_id.is_some() && dto.user_id.unwrap() != current_person.user_id.unwrap_or(-1) && claims.role != "admin" {
        return Err(AppError::Forbidden("Only admins can change user associations".to_string()));
    }

    let person = service.update_person(person_id, dto).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(person, "Person updated successfully")))
}

#[delete("/{id}")]
pub async fn delete_person(
    service: web::Data<Arc<PersonService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let person_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Get the current person to check ownership
    let current_person = service.get_person_by_id(person_id).await?;

    // Users can only delete their own person data unless they are admins
    if current_person.user_id.unwrap_or(-1) != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    service.delete_person(person_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success((), "Person deleted successfully")))
}

#[get("/user/{user_id}")]
pub async fn get_person_by_user_id(
    service: web::Data<Arc<PersonService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Users can only access their own person data unless they are admins
    if user_id != claims.user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let person = service.get_person_by_user_id(user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(person, "Person retrieved successfully")))
}

