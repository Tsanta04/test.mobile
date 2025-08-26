use crate::schema::person;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub firstname: Option<String>,
    pub date_birth: Option<NaiveDateTime>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = person)]
pub struct NewPerson {
    pub name: String,
    pub firstname: Option<String>,
    pub date_birth: Option<NaiveDateTime>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = person)]
pub struct UpdatePerson {
    pub name: Option<String>,
    pub firstname: Option<String>,
    pub date_birth: Option<NaiveDateTime>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

