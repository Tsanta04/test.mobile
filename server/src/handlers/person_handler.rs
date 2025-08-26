use crate::auth::ClaimsExtractor;
use crate::dto::common::ApiResponse;
use crate::dto::person::{CreatePersonDto, PersonResponseDto, UpdatePersonDto};
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::services::PersonService;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn create_person(
    pool: web::Data<PgPool>,
    person_dto: web::Json<CreatePersonDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate the person DTO
    person_dto.validate()?;

    // Check if the user is creating a person for themselves or has admin role
    let claims = req.claims()?;
    if let Some(user_id) = person_dto.user_id {
        if user_id != claims.user_id && claims.role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "You can only create a person for yourself".to_string(),
            ));
        }
    }

    // Create the person
    let person = PersonService::create_person(&pool, person_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Created().json(ApiResponse::success(
        PersonResponseDto::from(person),
        "Person created successfully",
    )))
}

pub async fn get_person_by_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let person_id = path.into_inner();

    // Get the person
    let person = PersonService::get_person_by_id(&pool, person_id).await?;

    // Check if the user is requesting their own data or has admin role
    let claims = req.claims()?;
    if let Some(user_id) = person.user_id {
        if user_id != claims.user_id && claims.role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "You can only access your own person data".to_string(),
            ));
        }
    }

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        PersonResponseDto::from(person),
        "Person retrieved successfully",
    )))
}

pub async fn get_person_by_user_id(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();

    // Check if the user is requesting their own data or has admin role
    let claims = req.claims()?;
    if user_id != claims.user_id && claims.role != UserRole::Admin {
        return Err(AppError::Forbidden(
            "You can only access your own person data".to_string(),
        ));
    }

    // Get the person
    let person = PersonService::get_person_by_user_id(&pool, user_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        PersonResponseDto::from(person),
        "Person retrieved successfully",
    )))
}

pub async fn get_all_persons(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Check if the user has admin role
    let claims = req.claims()?;
    if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can view all persons".to_string()));
    }

    // Get all persons
    let persons = PersonService::get_all_persons(&pool).await?;

    // Convert to response DTOs
    let person_dtos: Vec<PersonResponseDto> = persons.into_iter().map(PersonResponseDto::from).collect();

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        person_dtos,
        "Persons retrieved successfully",
    )))
}

pub async fn update_person(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    person_dto: web::Json<UpdatePersonDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let person_id = path.into_inner();

    // Validate the person DTO
    person_dto.validate()?;

    // Get the current person
    let current_person = PersonService::get_person_by_id(&pool, person_id).await?;

    // Check if the user is updating their own data or has admin role
    let claims = req.claims()?;
    if let Some(user_id) = current_person.user_id {
        if user_id != claims.user_id && claims.role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "You can only update your own person data".to_string(),
            ));
        }
    }

    // If the user is trying to change the user_id, check if they have admin role
    if let Some(new_user_id) = person_dto.user_id {
        if new_user_id != claims.user_id && claims.role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "You can only set your own user ID".to_string(),
            ));
        }
    }

    // Update the person
    let person = PersonService::update_person(&pool, person_id, person_dto.0).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        PersonResponseDto::from(person),
        "Person updated successfully",
    )))
}

pub async fn delete_person(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let person_id = path.into_inner();

    // Get the current person
    let current_person = PersonService::get_person_by_id(&pool, person_id).await?;

    // Check if the user is deleting their own data or has admin role
    let claims = req.claims()?;
    if let Some(user_id) = current_person.user_id {
        if user_id != claims.user_id && claims.role != UserRole::Admin {
            return Err(AppError::Forbidden(
                "You can only delete your own person data".to_string(),
            ));
        }
    } else if claims.role != UserRole::Admin {
        return Err(AppError::Forbidden("Only admins can delete this person".to_string()));
    }

    // Delete the person
    PersonService::delete_person(&pool, person_id).await?;

    // Return the response
    Ok(HttpResponse::Ok().json(ApiResponse::success(
        (),
        "Person deleted successfully",
    )))
}

