use crate::models::participant::Participant;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateParticipantDto {
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateParticipantDto {
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantResponseDto {
    pub id: i32,
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

impl From<Participant> for ParticipantResponseDto {
    fn from(participant: Participant) -> Self {
        Self {
            id: participant.id,
            discussion_id: participant.discussion_id,
            participant_id: participant.participant_id,
        }
    }
}

