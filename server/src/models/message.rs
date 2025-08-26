use crate::schema::message;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = message)]
#[diesel(belongs_to(crate::models::user::User, foreign_key = sender_id))]
#[diesel(belongs_to(crate::models::discussion::Discussion, foreign_key = discussion_id))]
pub struct Message {
    pub id: i32,
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = message)]
pub struct NewMessage {
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = message)]
pub struct MessageUpdate {
    pub content: Option<String>,
}

