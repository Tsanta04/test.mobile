use crate::dto::user_dto::{AuthResponseDto, CreateUserDto, LoginDto, UpdateUserDto, UserDto};
use crate::error::AppError;
use crate::services::user_service::UserService;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

#[get("/users")]
pub async fn get_all_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match UserService::find_all(&mut conn) {
        Ok(users) => {
            let user_dtos: Vec<UserDto> = users.into_iter().map(UserDto::from).collect();
            HttpResponse::Ok().json(user_dtos)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match UserService::find_by_id(id, &mut conn) {
        Ok(user) => HttpResponse::Ok().json(UserDto::from(user)),
        Err(e) => match e {
            AppError::NotFound(_) => HttpResponse::NotFound().body(format!("User with ID {} not found", id)),
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[post("/users")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    user_dto: web::Json<CreateUserDto>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(
        user_dto.password.as_bytes(),
        &salt,
    ) {
        Ok(hash) => hash.to_string(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
    };

    let new_user = user_dto.into_inner().to_new_user(password_hash);

    match UserService::create(new_user, &mut conn) {
        Ok(user) => HttpResponse::Created().json(UserDto::from(user)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    user_dto: web::Json<UpdateUserDto>,
) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Hash the password if provided
    let hashed_password = if let Some(ref password) = user_dto.password {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Some(hash.to_string()),
            Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
        }
    } else {
        None
    };

    let user_update = user_dto.into_inner().to_user_update(hashed_password);

    match UserService::update(id, user_update, &mut conn) {
        Ok(user) => HttpResponse::Ok().json(UserDto::from(user)),
        Err(e) => match e {
            AppError::NotFound(_) => HttpResponse::NotFound().body(format!("User with ID {} not found", id)),
            _ => HttpResponse::InternalServerError().body(e.to_string()),
        },
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match UserService::delete(id, &mut conn) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().body(format!("User with ID {} not found", id))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
pub async fn login(pool: web::Data<DbPool>, login_dto: web::Json<LoginDto>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Find user by username
    let user = match UserService::find_by_username(&login_dto.username, &mut conn) {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid username or password"),
    };

    // Verify password
    let parsed_hash = match PasswordHash::new(&user.hashed_password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to parse password hash"),
    };

    let argon2 = Argon2::default();
    let is_valid = argon2
        .verify_password(login_dto.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return HttpResponse::Unauthorized().body("Invalid username or password");
    }

    // Generate JWT token
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.user_id.to_string(),
        role: user.role.clone(),
        exp: expiration,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to generate token"),
    };

    // Return token and user info
    let auth_response = AuthResponseDto {
        token,
        user: UserDto::from(user),
    };

    HttpResponse::Ok().json(auth_response)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_all_users)
            .service(get_user_by_id)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(login),
    );
}

