use crate::models::user::{User, UserRole};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: Option<String>,
    
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: Option<String>,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: Option<String>,
    
    pub role: Option<UserRole>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub role: UserRole,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            role: user.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,
    
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub token: String,
    pub user: UserResponseDto,
}

