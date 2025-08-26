use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use argon2::password_hash;
use diesel::result::Error as DieselError;
use jsonwebtoken::errors::Error as JwtError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
    InternalServerError(String),
    DatabaseError(String),
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    success: bool,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            success: false,
            message: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => AppError::NotFound("Record not found".to_string()),
            DieselError::DatabaseError(_, info) => {
                AppError::DatabaseError(format!("Database error: {}", info.message()))
            }
            _ => AppError::InternalServerError(format!("Database error: {}", error)),
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> Self {
        AppError::InternalServerError(format!("Connection pool error: {}", error))
    }
}

impl From<JwtError> for AppError {
    fn from(error: JwtError) -> Self {
        AppError::Unauthorized(format!("JWT error: {}", error))
    }
}

impl From<password_hash::Error> for AppError {
    fn from(error: password_hash::Error) -> Self {
        AppError::InternalServerError(format!("Password hashing error: {}", error))
    }
}

impl From<std::env::VarError> for AppError {
    fn from(error: std::env::VarError) -> Self {
        AppError::InternalServerError(format!("Environment variable error: {}", error))
    }
}

impl std::error::Error for AppError {}

