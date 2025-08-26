use crate::models::ground::Ground;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateGroundDto {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: String,
    
    #[validate(length(min = 1, max = 200, message = "Description must be between 1 and 200 characters"))]
    pub description: String,
    
    #[validate(length(max = 200, message = "Folder must be less than 200 characters"))]
    pub folder: Option<String>,
    
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    
    #[validate(length(max = 200, message = "Pack ID must be less than 200 characters"))]
    pub pack: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateGroundDto {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: Option<String>,
    
    #[validate(length(min = 1, max = 200, message = "Description must be between 1 and 200 characters"))]
    pub description: Option<String>,
    
    #[validate(length(max = 200, message = "Folder must be less than 200 characters"))]
    pub folder: Option<String>,
    
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    
    #[validate(length(max = 200, message = "Pack ID must be less than 200 characters"))]
    pub pack: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroundResponseDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub folder: Option<String>,
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    pub pack: Option<String>,
}

impl From<Ground> for GroundResponseDto {
    fn from(ground: Ground) -> Self {
        Self {
            id: ground.id,
            name: ground.name,
            description: ground.description,
            folder: ground.folder,
            culture_type: ground.culture_type,
            location: ground.location,
            user_id: ground.user_id,
            pack: ground.pack,
        }
    }
}

