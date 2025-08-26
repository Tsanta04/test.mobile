use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningDto {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlanningDto {
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePlanningDto {
    pub date: Option<NaiveDateTime>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub ground: Option<i32>,
}

