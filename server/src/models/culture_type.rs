use crate::schema::culture_type;
use diesel::prelude::*;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = culture_type)]
pub struct CultureType {
    pub id: i32,
    pub type_: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = culture_type)]
pub struct NewCultureType {
    pub type_: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = culture_type)]
pub struct CultureTypeUpdate {
    pub type_: Option<String>,
}

