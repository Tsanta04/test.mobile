use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorDto {
    pub id: i32,
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSensorDto {
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSensorDto {
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

