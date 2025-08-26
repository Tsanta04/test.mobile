use crate::schema::message;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Message {
    pub id: i32,
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = message)]
pub struct NewMessage {
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = message)]
pub struct UpdateMessage {
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

