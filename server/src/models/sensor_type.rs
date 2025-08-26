use crate::schema::sensor_type;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = sensor_type)]
pub struct SensorType {
    pub id: i32,
    pub type_: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = sensor_type)]
pub struct NewSensorType {
    pub type_: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = sensor_type)]
pub struct UpdateSensorType {
    pub type_: Option<String>,
}

