use crate::schema::planning;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = planning)]
pub struct Planning {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = planning)]
pub struct NewPlanning {
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = planning)]
pub struct PlanningUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

