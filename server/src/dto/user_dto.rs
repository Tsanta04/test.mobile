use crate::models::user::{NewUser, User, UserRole, UserUpdate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            role: user.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl CreateUserDto {
    pub fn to_new_user(self, hashed_password: String) -> NewUser {
        NewUser {
            username: self.username,
            email: self.email,
            hashed_password,
            role: self.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}

impl UpdateUserDto {
    pub fn to_user_update(self, hashed_password: Option<String>) -> UserUpdate {
        UserUpdate {
            username: self.username,
            email: self.email,
            hashed_password,
            role: self.role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponseDto {
    pub token: String,
    pub user: UserDto,
}

