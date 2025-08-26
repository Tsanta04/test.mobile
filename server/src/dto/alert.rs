use crate::models::alert::{Alert, AlertType, LevelType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAlertDto {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: String,
    
    #[validate(length(min = 1, max = 200, message = "Description must be between 1 and 200 characters"))]
    pub description: String,
    
    pub type_: AlertType,
    pub level: Option<LevelType>,
    
    #[validate(length(max = 200, message = "Recommendation must be less than 200 characters"))]
    pub recommandation: Option<String>,
    
    pub is_seen: Option<bool>,
    pub state_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateAlertDto {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters"))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 200, message = "Description must be between 1 and 200 characters"))]
    pub description: Option<String>,
    
    pub type_: Option<AlertType>,
    pub level: Option<LevelType>,
    
    #[validate(length(max = 200, message = "Recommendation must be less than 200 characters"))]
    pub recommandation: Option<String>,
    
    pub is_seen: Option<bool>,
    pub state_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertResponseDto {
    pub id: i32,
    pub date: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub type_: AlertType,
    pub level: LevelType,
    pub recommandation: Option<String>,
    pub is_seen: bool,
    pub state_id: Option<i32>,
}

impl From<Alert> for AlertResponseDto {
    fn from(alert: Alert) -> Self {
        Self {
            id: alert.id,
            date: alert.date,
            title: alert.title,
            description: alert.description,
            type_: alert.type_,
            level: alert.level,
            recommandation: alert.recommandation,
            is_seen: alert.is_seen,
            state_id: alert.state_id,
        }
    }
}

