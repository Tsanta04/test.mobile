use crate::schema::ground;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = ground)]
pub struct Ground {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub folder: Option<String>,
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    pub pack: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = ground)]
pub struct NewGround {
    pub name: String,
    pub description: String,
    pub folder: Option<String>,
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    pub pack: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = ground)]
pub struct GroundUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub folder: Option<String>,
    pub culture_type: Option<i32>,
    pub location: Option<i32>,
    pub user_id: Option<i32>,
    pub pack: Option<String>,
}

