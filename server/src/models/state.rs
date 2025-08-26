use crate::schema::state;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct State {
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

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = state)]
pub struct NewState {
    pub date: Option<NaiveDateTime>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = state)]
pub struct UpdateState {
    pub date: Option<NaiveDateTime>,
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

