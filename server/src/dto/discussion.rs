use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscussionDto {
    pub id: i32,
    pub name: String,
    pub initialised_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDiscussionDto {
    pub name: String,
    pub initialised_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDiscussionDto {
    pub name: Option<String>,
    pub initialised_at: Option<NaiveDateTime>,
}

