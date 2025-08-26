use crate::models::sensor_type::SensorType;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSensorTypeDto {
    #[validate(length(min = 1, max = 200, message = "Type must be between 1 and 200 characters"))]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSensorTypeDto {
    #[validate(length(min = 1, max = 200, message = "Type must be between 1 and 200 characters"))]
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorTypeResponseDto {
    pub id: i32,
    pub type_: String,
}

impl From<SensorType> for SensorTypeResponseDto {
    fn from(sensor_type: SensorType) -> Self {
        Self {
            id: sensor_type.id,
            type_: sensor_type.type_,
        }
    }
}

