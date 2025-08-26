use crate::dto::{ApiResponse, CreateUserDto, LoginDto, UpdateUserDto, UserDto};
use crate::errors::AppError;
use crate::services::{Claims, UsersService};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;

#[post("/register")]
pub async fn register(
    service: web::Data<Arc<UsersService>>,
    user_dto: web::Json<CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    let user = service.create_user(user_dto.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse::success(user, "User created successfully")))
}

#[post("/login")]
pub async fn login(
    service: web::Data<Arc<UsersService>>,
    login_dto: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let token_response = service.login(login_dto.into_inner(), &jwt_secret).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(token_response, "Login successful")))
}

#[get("")]
pub async fn get_all_users(
    service: web::Data<Arc<UsersService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Check if user has admin role
    let claims = req.extensions().get::<Claims>().unwrap();
    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin access required".to_string()));
    }

    let users = service.get_all_users().await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(users, "Users retrieved successfully")))
}

#[get("/{id}")]
pub async fn get_user_by_id(
    service: web::Data<Arc<UsersService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Users can only access their own data unless they are admins
    if claims.user_id != user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let user = service.get_user_by_id(user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(user, "User retrieved successfully")))
}

#[put("/{id}")]
pub async fn update_user(
    service: web::Data<Arc<UsersService>>,
    path: web::Path<i32>,
    user_dto: web::Json<UpdateUserDto>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Users can only update their own data unless they are admins
    if claims.user_id != user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    // Only admins can change roles
    if user_dto.role.is_some() && claims.role != "admin" {
        return Err(AppError::Forbidden("Only admins can change roles".to_string()));
    }

    let user = service.update_user(user_id, user_dto.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(user, "User updated successfully")))
}

#[delete("/{id}")]
pub async fn delete_user(
    service: web::Data<Arc<UsersService>>,
    path: web::Path<i32>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = path.into_inner();
    let claims = req.extensions().get::<Claims>().unwrap();

    // Users can only delete their own account unless they are admins
    if claims.user_id != user_id && claims.role != "admin" {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    service.delete_user(user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::success((), "User deleted successfully")))
}

#[get("/me")]
pub async fn get_current_user(
    service: web::Data<Arc<UsersService>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = req.extensions().get::<Claims>().unwrap();
    let user = service.get_user_by_id(claims.user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::success(user, "Current user retrieved successfully")))
}

