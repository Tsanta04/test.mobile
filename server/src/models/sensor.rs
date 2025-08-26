use crate::schema::sensor;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = sensor)]
pub struct Sensor {
    pub id: i32,
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sensor)]
pub struct NewSensor {
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = sensor)]
pub struct SensorUpdate {
    pub description: Option<String>,
    pub issue_date: Option<NaiveDateTime>,
    pub sensor_type: Option<i32>,
}

