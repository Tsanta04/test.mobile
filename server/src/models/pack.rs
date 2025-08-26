use crate::schema::pack;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Pack {
    pub id: i32,
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = pack)]
pub struct NewPack {
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = pack)]
pub struct UpdatePack {
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

