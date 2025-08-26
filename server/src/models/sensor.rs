use crate::schema::sensor;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Sensor {
    pub id: i32,
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = sensor)]
pub struct NewSensor {
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = sensor)]
pub struct UpdateSensor {
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

