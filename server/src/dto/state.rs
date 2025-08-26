use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StateDto {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStateDto {
    pub date: Option<NaiveDateTime>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStateDto {
    pub date: Option<NaiveDateTime>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

