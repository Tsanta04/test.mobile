use crate::dto::{CreateUserDto, LoginDto, TokenResponse, UpdateUserDto, UserDto};
use crate::errors::AppError;
use crate::models::{NewUser, UpdateUser, User};
use crate::schema::users;
use crate::services::base_service::{BaseService, DbPool, Service};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct UsersService {
    base: Service,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub user_id: i32,
    pub role: String,
}

impl UsersService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_user(&self, dto: CreateUserDto) -> Result<UserDto, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if user with same email or username exists
        let existing_user = users
            .filter(email.eq(&dto.email).or(username.eq(&dto.username)))
            .first::<User>(conn)
            .optional()?;

        if let Some(user) = existing_user {
            if user.email == dto.email {
                return Err(AppError::BadRequest("Email already exists".to_string()));
            } else {
                return Err(AppError::BadRequest("Username already exists".to_string()));
            }
        }

        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(dto.password.as_bytes(), &salt)?
            .to_string();

        let new_user = NewUser {
            username: dto.username,
            email: dto.email,
            hashed_password: password_hash,
            role: dto.role,
        };

        let user = diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(conn)?;

        Ok(UserDto {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            role: user.role,
        })
    }

    pub async fn login(&self, dto: LoginDto, jwt_secret: &str) -> Result<TokenResponse, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let user = users
            .filter(email.eq(&dto.email))
            .first::<User>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Verify password
        let parsed_hash = PasswordHash::new(&user.hashed_password)?;
        Argon2::default()
            .verify_password(dto.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // Generate JWT token
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.email.clone(),
            exp: expiration as usize,
            user_id: user.user_id,
            role: user.role.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )?;

        Ok(TokenResponse {
            token,
            user: UserDto {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                role: user.role,
            },
        })
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<UserDto, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let user = users
            .find(id)
            .first::<User>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserDto {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            role: user.role,
        })
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserDto>, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = users.load::<User>(conn)?;

        let user_dtos = results
            .into_iter()
            .map(|user| UserDto {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                role: user.role,
            })
            .collect();

        Ok(user_dtos)
    }

    pub async fn update_user(&self, id: i32, dto: UpdateUserDto) -> Result<UserDto, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if user exists
        let user = users
            .find(id)
            .first::<User>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // If email is being updated, check if it's already in use
        if let Some(ref email_val) = dto.email {
            if email_val != &user.email {
                let email_exists = users
                    .filter(email.eq(email_val))
                    .first::<User>(conn)
                    .optional()?
                    .is_some();

                if email_exists {
                    return Err(AppError::BadRequest("Email already exists".to_string()));
                }
            }
        }

        // If username is being updated, check if it's already in use
        if let Some(ref username_val) = dto.username {
            if username_val != &user.username {
                let username_exists = users
                    .filter(username.eq(username_val))
                    .first::<User>(conn)
                    .optional()?
                    .is_some();

                if username_exists {
                    return Err(AppError::BadRequest("Username already exists".to_string()));
                }
            }
        }

        // Hash password if provided
        let hashed_password_val = if let Some(password_val) = dto.password {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            Some(
                argon2
                    .hash_password(password_val.as_bytes(), &salt)?
                    .to_string(),
            )
        } else {
            None
        };

        let update_user = UpdateUser {
            username: dto.username,
            email: dto.email,
            hashed_password: hashed_password_val,
            role: dto.role,
        };

        let updated_user = diesel::update(users.find(id))
            .set(&update_user)
            .get_result::<User>(conn)?;

        Ok(UserDto {
            user_id: updated_user.user_id,
            username: updated_user.username,
            email: updated_user.email,
            role: updated_user.role,
        })
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if user exists
        let user_exists = users
            .find(id)
            .first::<User>(conn)
            .optional()?
            .is_some();

        if !user_exists {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        diesel::delete(users.find(id)).execute(conn)?;

        Ok(())
    }
}

impl BaseService for UsersService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

