use crate::schema::culture_type;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = culture_type)]
pub struct CultureType {
    pub id: i32,
    pub type_: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = culture_type)]
pub struct NewCultureType {
    pub type_: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = culture_type)]
pub struct UpdateCultureType {
    pub type_: Option<String>,
}

