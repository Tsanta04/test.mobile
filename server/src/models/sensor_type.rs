use crate::schema::sensor_type;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = sensor_type)]
pub struct SensorType {
    pub id: i32,
    pub type_: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sensor_type)]
pub struct NewSensorType {
    pub type_: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = sensor_type)]
pub struct SensorTypeUpdate {
    pub type_: Option<String>,
}

