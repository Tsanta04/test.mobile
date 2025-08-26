use crate::models::sensor_pack::SensorPack;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSensorPackDto {
    #[validate(length(min = 1, max = 200, message = "ID must be between 1 and 200 characters"))]
    pub id: String,
    
    #[validate(length(max = 200, message = "Description must be less than 200 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSensorPackDto {
    #[validate(length(max = 200, message = "Description must be less than 200 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorPackResponseDto {
    pub id: String,
    pub description: Option<String>,
}

impl From<SensorPack> for SensorPackResponseDto {
    fn from(sensor_pack: SensorPack) -> Self {
        Self {
            id: sensor_pack.id,
            description: sensor_pack.description,
        }
    }
}

