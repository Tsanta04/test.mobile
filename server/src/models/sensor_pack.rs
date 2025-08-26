use crate::schema::sensor_pack;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = sensor_pack, primary_key(id))]
pub struct SensorPack {
    pub id: String,
    pub description: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = sensor_pack)]
pub struct NewSensorPack {
    pub id: String,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = sensor_pack, primary_key(id))]
pub struct SensorPackUpdate {
    pub description: Option<String>,
}

