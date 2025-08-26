use crate::models::alert::{AlertType, AlertUpdate, LevelType, NewAlert, Alert};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertDto {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub type_: String,
    pub level: String,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

impl From<Alert> for AlertDto {
    fn from(alert: Alert) -> Self {
        AlertDto {
            id: alert.id,
            date: alert.date,
            title: alert.title,
            description: alert.description,
            type_: format!("{:?}", alert.type_),
            level: format!("{:?}", alert.level),
            recommandation: alert.recommandation,
            isseen: alert.isseen,
            state_id: alert.state_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAlertDto {
    pub title: String,
    pub description: String,
    pub type_: String,
    pub level: Option<String>,
    pub recommandation: Option<String>,
    pub state_id: Option<i32>,
}

impl CreateAlertDto {
    pub fn to_new_alert(self) -> NewAlert {
        let alert_type = match self.type_.as_str() {
            "Health" => AlertType::Health,
            "Production" => AlertType::Production,
            "Rentability" => AlertType::Rentability,
            "Humidity" => AlertType::Humidity,
            "Fertility" => AlertType::Fertility,
            _ => AlertType::Other,
        };

        let level = match self.level {
            Some(ref level) => match level.as_str() {
                "Medium" => LevelType::Medium,
                "High" => LevelType::High,
                "Urgent" => LevelType::Urgent,
                _ => LevelType::Low,
            },
            None => LevelType::Low,
        };

        NewAlert {
            date: Some(chrono::Utc::now().naive_utc()),
            title: self.title,
            description: self.description,
            type_: alert_type,
            level,
            recommandation: self.recommandation,
            isseen: Some(false),
            state_id: self.state_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAlertDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub type_: Option<String>,
    pub level: Option<String>,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

impl UpdateAlertDto {
    pub fn to_alert_update(self) -> AlertUpdate {
        let alert_type = self.type_.map(|t| match t.as_str() {
            "Health" => AlertType::Health,
            "Production" => AlertType::Production,
            "Rentability" => AlertType::Rentability,
            "Humidity" => AlertType::Humidity,
            "Fertility" => AlertType::Fertility,
            _ => AlertType::Other,
        });

        let level = self.level.map(|l| match l.as_str() {
            "Medium" => LevelType::Medium,
            "High" => LevelType::High,
            "Urgent" => LevelType::Urgent,
            _ => LevelType::Low,
        });

        AlertUpdate {
            title: self.title,
            description: self.description,
            type_: alert_type,
            level,
            recommandation: self.recommandation,
            isseen: self.isseen,
            state_id: self.state_id,
        }
    }
}

