use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantDto {
    pub id: i32,
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParticipantDto {
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateParticipantDto {
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

