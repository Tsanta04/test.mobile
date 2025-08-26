use crate::dto::{CreateParticipantDto, ParticipantDto, UpdateParticipantDto};
use crate::errors::AppError;
use crate::models::{NewParticipant, Participant, UpdateParticipant};
use crate::schema::participant;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct ParticipantService {
    base: Service,
}

impl ParticipantService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_participant(
        &self,
        dto: CreateParticipantDto,
    ) -> Result<ParticipantDto, AppError> {
        use crate::schema::participant::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_participant = NewParticipant {
            discussion_id: dto.discussion_id,
            participant_id: dto.participant_id,
        };

        let participant_result = diesel::insert_into(participant)
            .values(&new_participant)
            .get_result::<Participant>(conn)?;

        Ok(ParticipantDto {
            id: participant_result.id,
            discussion_id: participant_result.discussion_id,
            participant_id: participant_result.participant_id,
        })
    }

    pub async fn get_participant_by_id(&self, participant_id: i32) -> Result<ParticipantDto, AppError> {
        use crate::schema::participant::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let participant_result = participant
            .find(participant_id)
            .first::<Participant>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Participant not found".to_string()))?;

        Ok(ParticipantDto {
            id: participant_result.id,
            discussion_id: participant_result.discussion_id,
            participant_id: participant_result.participant_id,
        })
    }

    pub async fn get_all_participants(&self) -> Result<Vec<ParticipantDto>, AppError> {
        use crate::schema::participant::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = participant.load::<Participant>(conn)?;

        let participant_dtos = results
            .into_iter()
            .map(|p| ParticipantDto {
                id: p.id,
                discussion_id: p.discussion_id,
                participant_id: p.participant_id,
            })
            .collect();

        Ok(participant_dtos)
    }

    pub async fn update_participant(
        &self,
        participant_id_val: i32,
        dto: UpdateParticipantDto,
    ) -> Result<ParticipantDto, AppError> {
        use crate::schema::participant::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if participant exists
        let participant_exists = participant
            .find(participant_id_val)
            .first::<Participant>(conn)
            .optional()?
            .is_some();

        if !participant_exists {
            return Err(AppError::NotFound("Participant not found".to_string()));
        }

        let update_participant = UpdateParticipant {
            discussion_id: dto.discussion_id,
            participant_id: dto.participant_id,
        };

        let updated_participant = diesel::update(participant.find(participant_id_val))
            .set(&update_participant)
            .get_result::<Participant>(conn)?;

        Ok(ParticipantDto {
            id: updated_participant.id,
            discussion_id: updated_participant.discussion_id,
            participant_id: updated_participant.participant_id,
        })
    }

    pub async fn delete_participant(&self, participant_id_val: i32) -> Result<(), AppError> {
        use crate::schema::participant::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if participant exists
        let participant_exists = participant
            .find(participant_id_val)
            .first::<Participant>(conn)
            .optional()?
            .is_some();

        if !participant_exists {
            return Err(AppError::NotFound("Participant not found".to_string()));
        }

        diesel::delete(participant.find(participant_id_val)).execute(conn)?;

        Ok(())
    }

    pub async fn get_participants_by_discussion_id(
        &self,
        discussion_id_val: i32,
    ) -> Result<Vec<ParticipantDto>, AppError> {
        use crate::schema::participant::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = participant
            .filter(discussion_id.eq(discussion_id_val))
            .load::<Participant>(conn)?;

        let participant_dtos = results
            .into_iter()
            .map(|p| ParticipantDto {
                id: p.id,
                discussion_id: p.discussion_id,
                participant_id: p.participant_id,
            })
            .collect();

        Ok(participant_dtos)
    }

    pub async fn get_discussions_by_participant_id(
        &self,
        user_id_val: i32,
    ) -> Result<Vec<ParticipantDto>, AppError> {
        use crate::schema::participant::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = participant
            .filter(participant_id.eq(user_id_val))
            .load::<Participant>(conn)?;

        let participant_dtos = results
            .into_iter()
            .map(|p| ParticipantDto {
                id: p.id,
                discussion_id: p.discussion_id,
                participant_id: p.participant_id,
            })
            .collect();

        Ok(participant_dtos)
    }
}

impl BaseService for ParticipantService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

