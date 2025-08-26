use crate::dto::message::{CreateMessageDto, UpdateMessageDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::message::{Message, NewMessage, MessageUpdate};
use sqlx::PgPool;

pub struct MessageService;

impl MessageService {
    pub async fn create_message(
        pool: &PgPool,
        message_dto: CreateMessageDto,
    ) -> Result<Message, AppError> {
        // Create a new message
        let new_message = NewMessage {
            sender_id: message_dto.sender_id,
            content: message_dto.content,
            discussion_id: message_dto.discussion_id,
        };

        // Insert the message into the database
        let message = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO Message (sender_id, content, discussion_id)
            VALUES ($1, $2, $3)
            RETURNING id, sender_id, content, discussion_id
            "#,
            new_message.sender_id,
            new_message.content,
            new_message.discussion_id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(message)
    }

    pub async fn get_message_by_id(pool: &PgPool, id: i32) -> Result<Message, AppError> {
        let message = sqlx::query_as!(
            Message,
            r#"
            SELECT id, sender_id, content, discussion_id
            FROM Message
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(message)
    }

    pub async fn get_messages_by_discussion(pool: &PgPool, discussion_id: i32) -> Result<Vec<Message>, AppError> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, sender_id, content, discussion_id
            FROM Message
            WHERE discussion_id = $1
            ORDER BY id
            "#,
            discussion_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(messages)
    }

    pub async fn get_messages_by_sender(pool: &PgPool, sender_id: i32) -> Result<Vec<Message>, AppError> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, sender_id, content, discussion_id
            FROM Message
            WHERE sender_id = $1
            ORDER BY id
            "#,
            sender_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(messages)
    }

    pub async fn get_all_messages(pool: &PgPool) -> Result<Vec<Message>, AppError> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, sender_id, content, discussion_id
            FROM Message
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(messages)
    }

    pub async fn update_message(
        pool: &PgPool,
        id: i32,
        message_dto: UpdateMessageDto,
    ) -> Result<Message, AppError> {
        // Prepare the update
        let message_update = MessageUpdate {
            content: message_dto.content,
        };

        // Update the message
        let message = sqlx::query_as!(
            Message,
            r#"
            UPDATE Message
            SET 
                content = COALESCE($1, content)
            WHERE id = $2
            RETURNING id, sender_id, content, discussion_id
            "#,
            message_update.content,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(message)
    }

    pub async fn delete_message(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Message
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

