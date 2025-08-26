use crate::schema::person;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = person)]
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

#[derive(Debug, Insertable)]
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

#[derive(Debug, AsChangeset)]
#[diesel(table_name = person)]
pub struct PersonUpdate {
    pub name: Option<String>,
    pub firstname: Option<String>,
    pub date_birth: Option<NaiveDateTime>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

