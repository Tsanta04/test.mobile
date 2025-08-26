use crate::schema::location;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Location {
    pub id: i32,
    pub longitude: f64,
    pub latitude: f64,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = location)]
pub struct NewLocation {
    pub longitude: f64,
    pub latitude: f64,
    pub city: String,
    pub country: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = location)]
pub struct UpdateLocation {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub city: Option<String>,
    pub country: Option<String>,
}

