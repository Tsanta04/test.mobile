use crate::schema::alert;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = AlertTypeMapping)]
pub enum AlertType {
    Health,
    Production,
    Rentability,
    Humidity,
    Fertility,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = LevelTypeMapping)]
pub enum LevelType {
    Low,
    Medium,
    High,
    Urgent,
}

// Custom SQL types for our enums
#[derive(SqlType)]
#[diesel(postgres_type(name = "alerttype"))]
pub struct AlertTypeMapping;

#[derive(SqlType)]
#[diesel(postgres_type(name = "leveltype"))]
pub struct LevelTypeMapping;

// Implement FromSql and ToSql for our enums
impl<DB> diesel::serialize::ToSql<AlertTypeMapping, DB> for AlertType
where
    DB: diesel::backend::Backend,
    String: diesel::serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, DB>) -> diesel::serialize::Result {
        let s = match *self {
            AlertType::Health => "Health",
            AlertType::Production => "Production",
            AlertType::Rentability => "Rentability",
            AlertType::Humidity => "Humidity",
            AlertType::Fertility => "Fertility",
            AlertType::Other => "Other",
        };
        s.to_sql(out)
    }
}

impl<DB> diesel::deserialize::FromSql<AlertTypeMapping, DB> for AlertType
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<Text, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        match s.as_str() {
            "Health" => Ok(AlertType::Health),
            "Production" => Ok(AlertType::Production),
            "Rentability" => Ok(AlertType::Rentability),
            "Humidity" => Ok(AlertType::Humidity),
            "Fertility" => Ok(AlertType::Fertility),
            "Other" => Ok(AlertType::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl<DB> diesel::serialize::ToSql<LevelTypeMapping, DB> for LevelType
where
    DB: diesel::backend::Backend,
    String: diesel::serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, DB>) -> diesel::serialize::Result {
        let s = match *self {
            LevelType::Low => "Low",
            LevelType::Medium => "Medium",
            LevelType::High => "High",
            LevelType::Urgent => "Urgent",
        };
        s.to_sql(out)
    }
}

impl<DB> diesel::deserialize::FromSql<LevelTypeMapping, DB> for LevelType
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<Text, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, DB>) -> diesel::deserialize::Result<Self> {
        let s = String::from_sql(bytes)?;
        match s.as_str() {
            "Low" => Ok(LevelType::Low),
            "Medium" => Ok(LevelType::Medium),
            "High" => Ok(LevelType::High),
            "Urgent" => Ok(LevelType::Urgent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = alert)]
#[diesel(belongs_to(crate::models::state::State, foreign_key = state_id))]
pub struct Alert {
    pub id: i32,
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub type_: AlertType,
    pub level: LevelType,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = alert)]
pub struct NewAlert {
    pub date: Option<NaiveDateTime>,
    pub title: String,
    pub description: String,
    pub type_: AlertType,
    pub level: LevelType,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = alert)]
pub struct AlertUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub type_: Option<AlertType>,
    pub level: Option<LevelType>,
    pub recommandation: Option<String>,
    pub isseen: Option<bool>,
    pub state_id: Option<i32>,
}

