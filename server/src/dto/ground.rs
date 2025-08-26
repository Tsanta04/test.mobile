use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub folder: Option<String>,
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    pub pack: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroundDto {
    pub name: String,
    pub description: String,
    pub folder: Option<String>,
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    pub pack: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroundDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub folder: Option<String>,
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    pub pack: Option<String>,
}

