use crate::models::ground::{Ground, GroundUpdate, NewGround};
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

impl From<Ground> for GroundDto {
    fn from(ground: Ground) -> Self {
        GroundDto {
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

impl CreateGroundDto {
    pub fn to_new_ground(self) -> NewGround {
        NewGround {
            name: self.name,
            description: self.description,
            folder: self.folder,
            culture_type: self.culture_type,
            location: self.location,
            user_id: self.user_id,
            pack: self.pack,
        }
    }
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

impl UpdateGroundDto {
    pub fn to_ground_update(self) -> GroundUpdate {
        GroundUpdate {
            name: self.name,
            description: self.description,
            folder: self.folder,
            culture_type: self.culture_type,
            location: self.location,
            user_id: self.user_id,
            pack: self.pack,
        }
    }
}

