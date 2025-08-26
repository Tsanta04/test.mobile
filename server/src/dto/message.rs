use crate::models::message::Message;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateMessageDto {
    pub sender_id: Option<i32>,
    
    #[validate(length(max = 200, message = "Content must be less than 200 characters"))]
    pub content: Option<String>,
    
    pub discussion_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateMessageDto {
    #[validate(length(max = 200, message = "Content must be less than 200 characters"))]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponseDto {
    pub id: i32,
    pub sender_id: Option<i32>,
    pub content: Option<String>,
    pub discussion_id: Option<i32>,
}

impl From<Message> for MessageResponseDto {
    fn from(message: Message) -> Self {
        Self {
            id: message.id,
            sender_id: message.sender_id,
            content: message.content,
            discussion_id: message.discussion_id,
        }
    }
}

