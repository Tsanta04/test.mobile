use crate::auth::jwt::create_token;
use crate::dto::common::ApiResponse;
use crate::dto::user::{LoginDto, LoginResponseDto, UserResponseDto};
use crate::error::AppError;
use crate::services::UserService;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

pub async fn login(
    pool: web::Data<PgPool>,
    login_dto: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    // Validate the login DTO
    login_dto.validate()?;

    // Get the user by username
    let user = UserService::get_user_by_username(&pool, &login_dto.username).await?;

    // Verify the password
    if !UserService::verify_password(&user, &login_dto.password).await? {
        return Err(AppError::Unauthorized("Invalid username or password".to_string()));
    }

    // Create a JWT token
    let token = create_token(&user)?;

    // Create the response
    let response = LoginResponseDto {
        token,
        user: UserResponseDto::from(user),
    };

    Ok(HttpResponse::Ok().json(ApiResponse::success(response, "Login successful")))
}

pub async fn register(
    pool: web::Data<PgPool>,
    user_dto: web::Json<crate::dto::user::CreateUserDto>,
) -> Result<HttpResponse, AppError> {
    // Validate the user DTO
    user_dto.validate()?;

    // Create the user
    let user = UserService::create_user(&pool, user_dto.0).await?;

    // Create a JWT token
    let token = create_token(&user)?;

    // Create the response
    let response = LoginResponseDto {
        token,
        user: UserResponseDto::from(user),
    };

    Ok(HttpResponse::Created().json(ApiResponse::success(response, "User registered successfully")))
}

