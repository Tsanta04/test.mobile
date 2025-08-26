use crate::schema::planning;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Planning {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = planning)]
pub struct NewPlanning {
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = planning)]
pub struct UpdatePlanning {
    pub date: Option<NaiveDateTime>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

