use crate::schema::discussion;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Discussion {
    pub id: i32,
    pub name: String,
    pub initialised_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = discussion)]
pub struct NewDiscussion {
    pub name: String,
    pub initialised_at: Option<NaiveDateTime>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = discussion)]
pub struct UpdateDiscussion {
    pub name: Option<String>,
    pub initialised_at: Option<NaiveDateTime>,
}

