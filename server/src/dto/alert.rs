use crate::models::types::{AlertType, LevelType};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertDto {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: AlertType,
    pub level: Option<LevelType>,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAlertDto {
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: AlertType,
    pub level: Option<LevelType>,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAlertDto {
    pub date: Option<NaiveDateTime>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<AlertType>,
    pub level: Option<LevelType>,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

