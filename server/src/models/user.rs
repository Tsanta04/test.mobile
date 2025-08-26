use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Seller,
    Buyer,
    Supplier,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Seller => write!(f, "seller"),
            UserRole::Buyer => write!(f, "buyer"),
            UserRole::Supplier => write!(f, "supplier"),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "seller" => UserRole::Seller,
            "buyer" => UserRole::Buyer,
            "supplier" => UserRole::Supplier,
            _ => UserRole::Buyer, // Default role
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
    pub role: Option<String>,
}

