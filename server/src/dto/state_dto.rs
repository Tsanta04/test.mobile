use crate::models::state::{NewState, State, StateUpdate};
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

impl From<State> for StateDto {
    fn from(state: State) -> Self {
        StateDto {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStateDto {
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

impl CreateStateDto {
    pub fn to_new_state(self) -> NewState {
        NewState {
            date: Some(chrono::Utc::now().naive_utc()),
            temperature: self.temperature,
            health: self.health,
            production_progress: self.production_progress,
            humidity: self.humidity,
            fertility: self.fertility,
            rentability: self.rentability,
            pack_id: self.pack_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStateDto {
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

impl UpdateStateDto {
    pub fn to_state_update(self) -> StateUpdate {
        StateUpdate {
            temperature: self.temperature,
            health: self.health,
            production_progress: self.production_progress,
            humidity: self.humidity,
            fertility: self.fertility,
            rentability: self.rentability,
            pack_id: self.pack_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRangeDto {
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

