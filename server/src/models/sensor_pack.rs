use crate::schema::sensor_pack;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(primary_key(id))]
pub struct SensorPack {
    pub id: String,
    pub description: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = sensor_pack)]
pub struct NewSensorPack {
    pub id: String,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = sensor_pack)]
pub struct UpdateSensorPack {
    pub description: Option<String>,
}

