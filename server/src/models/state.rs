use crate::schema::state;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = state)]
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

#[derive(Debug, Insertable)]
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

#[derive(Debug, AsChangeset)]
#[diesel(table_name = state)]
pub struct StateUpdate {
    pub temperature: Option<f64>,
    pub health: Option<f64>,
    pub production_progress: Option<f64>,
    pub humidity: Option<f64>,
    pub fertility: Option<f64>,
    pub rentability: Option<f64>,
    pub pack_id: Option<String>,
}

