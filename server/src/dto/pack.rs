use crate::models::pack::Pack;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePackDto {
    #[validate(length(max = 200, message = "Pack ID must be less than 200 characters"))]
    pub pack_id: Option<String>,
    
    pub sensor_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePackDto {
    #[validate(length(max = 200, message = "Pack ID must be less than 200 characters"))]
    pub pack_id: Option<String>,
    
    pub sensor_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackResponseDto {
    pub id: i32,
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

impl From<Pack> for PackResponseDto {
    fn from(pack: Pack) -> Self {
        Self {
            id: pack.id,
            pack_id: pack.pack_id,
            sensor_id: pack.sensor_id,
        }
    }
}

