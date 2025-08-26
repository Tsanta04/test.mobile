use crate::models::planning::Planning;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePlanningDto {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: String,
    
    #[validate(length(min = 1, max = 200, message = "Description must be between 1 and 200 characters"))]
    pub description: String,
    
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub ground: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePlanningDto {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 200, message = "Description must be between 1 and 200 characters"))]
    pub description: Option<String>,
    
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub ground: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanningResponseDto {
    pub id: i32,
    pub date: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub ground: Option<i32>,
}

impl From<Planning> for PlanningResponseDto {
    fn from(planning: Planning) -> Self {
        Self {
            id: planning.id,
            date: planning.date,
            title: planning.title,
            description: planning.description,
            start_date: planning.start_date,
            end_date: planning.end_date,
            ground: planning.ground,
        }
    }
}

