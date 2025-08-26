use crate::schema::discussion;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = discussion)]
pub struct Discussion {
    pub id: i32,
    pub name: String,
    pub initialised_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = discussion)]
pub struct NewDiscussion {
    pub name: String,
    pub initialised_at: Option<NaiveDateTime>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = discussion)]
pub struct DiscussionUpdate {
    pub name: Option<String>,
}

