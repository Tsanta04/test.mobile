use crate::dto::participant::{CreateParticipantDto, UpdateParticipantDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::participant::{Participant, NewParticipant, ParticipantUpdate};
use sqlx::PgPool;

pub struct ParticipantService;

impl ParticipantService {
    pub async fn create_participant(
        pool: &PgPool,
        participant_dto: CreateParticipantDto,
    ) -> Result<Participant, AppError> {
        // Create a new participant
        let new_participant = NewParticipant {
            discussion_id: participant_dto.discussion_id,
            participant_id: participant_dto.participant_id,
        };

        // Insert the participant into the database
        let participant = sqlx::query_as!(
            Participant,
            r#"
            INSERT INTO Participant (discussion_id, participant_id)
            VALUES ($1, $2)
            RETURNING id, discussion_id, participant_id
            "#,
            new_participant.discussion_id,
            new_participant.participant_id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(participant)
    }

    pub async fn get_participant_by_id(pool: &PgPool, id: i32) -> Result<Participant, AppError> {
        let participant = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, discussion_id, participant_id
            FROM Participant
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(participant)
    }

    pub async fn get_participants_by_discussion(pool: &PgPool, discussion_id: i32) -> Result<Vec<Participant>, AppError> {
        let participants = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, discussion_id, participant_id
            FROM Participant
            WHERE discussion_id = $1
            ORDER BY id
            "#,
            discussion_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(participants)
    }

    pub async fn get_participants_by_user(pool: &PgPool, user_id: i32) -> Result<Vec<Participant>, AppError> {
        let participants = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, discussion_id, participant_id
            FROM Participant
            WHERE participant_id = $1
            ORDER BY id
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(participants)
    }

    pub async fn get_all_participants(pool: &PgPool) -> Result<Vec<Participant>, AppError> {
        let participants = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, discussion_id, participant_id
            FROM Participant
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(participants)
    }

    pub async fn update_participant(
        pool: &PgPool,
        id: i32,
        participant_dto: UpdateParticipantDto,
    ) -> Result<Participant, AppError> {
        // Prepare the update
        let participant_update = ParticipantUpdate {
            discussion_id: participant_dto.discussion_id,
            participant_id: participant_dto.participant_id,
        };

        // Update the participant
        let participant = sqlx::query_as!(
            Participant,
            r#"
            UPDATE Participant
            SET 
                discussion_id = COALESCE($1, discussion_id),
                participant_id = COALESCE($2, participant_id)
            WHERE id = $3
            RETURNING id, discussion_id, participant_id
            "#,
            participant_update.discussion_id,
            participant_update.participant_id,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(participant)
    }

    pub async fn delete_participant(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Participant
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    pub async fn delete_participant_by_user_and_discussion(
        pool: &PgPool,
        user_id: i32,
        discussion_id: i32,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Participant
            WHERE participant_id = $1 AND discussion_id = $2
            "#,
            user_id,
            discussion_id
        )
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

