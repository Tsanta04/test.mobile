use crate::models::discussion::Discussion;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateDiscussionDto {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateDiscussionDto {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscussionResponseDto {
    pub id: i32,
    pub name: String,
    pub initialised_at: DateTime<Utc>,
}

impl From<Discussion> for DiscussionResponseDto {
    fn from(discussion: Discussion) -> Self {
        Self {
            id: discussion.id,
            name: discussion.name,
            initialised_at: discussion.initialised_at,
        }
    }
}

