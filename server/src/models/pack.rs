use crate::schema::pack;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = pack)]
pub struct Pack {
    pub id: i32,
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = pack)]
pub struct NewPack {
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = pack)]
pub struct PackUpdate {
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

