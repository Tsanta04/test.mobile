use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorTypeDto {
    pub id: i32,
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSensorTypeDto {
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSensorTypeDto {
    pub type_: Option<String>,
}

