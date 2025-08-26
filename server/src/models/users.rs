use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = users, primary_key(user_id))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
    pub role: Option<String>,
}

