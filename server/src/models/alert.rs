use crate::models::types::{AlertType, LevelType};
use crate::schema::alert;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Alert {
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

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = alert)]
pub struct NewAlert {
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

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = alert)]
pub struct UpdateAlert {
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

