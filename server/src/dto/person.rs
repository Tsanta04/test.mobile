use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonDto {
    pub id: i32,
    pub name: String,
    pub firstname: Option<String>,
    pub date_birth: Option<NaiveDateTime>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePersonDto {
    pub name: String,
    pub firstname: Option<String>,
    pub date_birth: Option<NaiveDateTime>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePersonDto {
    pub name: Option<String>,
    pub firstname: Option<String>,
    pub date_birth: Option<NaiveDateTime>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

