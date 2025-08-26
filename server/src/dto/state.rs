use crate::models::state::State;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateStateDto {
    pub date: Option<DateTime<Utc>>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    
    #[validate(length(max = 200, message = "Pack ID must be less than 200 characters"))]
    pub pack_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateStateDto {
    pub date: Option<DateTime<Utc>>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    
    #[validate(length(max = 200, message = "Pack ID must be less than 200 characters"))]
    pub pack_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateResponseDto {
    pub id: i32,
    pub date: DateTime<Utc>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

impl From<State> for StateResponseDto {
    fn from(state: State) -> Self {
        Self {
            id: state.id,
            date: state.date,
            temperature: state.temperature,
            health: state.health,
            production_progress: state.production_progress,
            humidity: state.humidity,
            fertility: state.fertility,
            rentability: state.rentability,
            pack_id: state.pack_id,
        }
    }
}

