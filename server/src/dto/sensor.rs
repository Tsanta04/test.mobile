use crate::models::sensor::Sensor;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSensorDto {
    #[validate(length(max = 200, message = "Description must be less than 200 characters"))]
    pub description: Option<String>,
    
    pub issue_date: Option<DateTime<Utc>>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSensorDto {
    #[validate(length(max = 200, message = "Description must be less than 200 characters"))]
    pub description: Option<String>,
    
    pub issue_date: Option<DateTime<Utc>>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorResponseDto {
    pub id: i32,
    pub description: Option<String>,
    pub issue_date: DateTime<Utc>,
    pub sensor_type: Option<i32>,
}

impl From<Sensor> for SensorResponseDto {
    fn from(sensor: Sensor) -> Self {
        Self {
            id: sensor.id,
            description: sensor.description,
            issue_date: sensor.issue_date,
            sensor_type: sensor.sensor_type,
        }
    }
}

