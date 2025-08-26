use crate::schema::location;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = location)]
pub struct Location {
    pub id: i32,
    pub longitude: f64,
    pub latitude: f64,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = location)]
pub struct NewLocation {
    pub longitude: f64,
    pub latitude: f64,
    pub city: String,
    pub country: String,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = location)]
pub struct LocationUpdate {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub city: Option<String>,
    pub country: Option<String>,
}

