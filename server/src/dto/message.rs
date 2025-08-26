use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDto {
    pub id: i32,
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageDto {
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageDto {
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

