use crate::dto::{CreateMessageDto, MessageDto, UpdateMessageDto};
use crate::errors::AppError;
use crate::models::{Message, NewMessage, UpdateMessage};
use crate::schema::message;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct MessageService {
    base: Service,
}

impl MessageService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_message(&self, dto: CreateMessageDto) -> Result<MessageDto, AppError> {
        use crate::schema::message::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_message = NewMessage {
            sender_id: dto.sender_id,
            content: dto.content,
            discussion_id: dto.discussion_id,
        };

        let message_result = diesel::insert_into(message)
            .values(&new_message)
            .get_result::<Message>(conn)?;

        Ok(MessageDto {
            id: message_result.id,
            sender_id: message_result.sender_id,
            content: message_result.content,
            discussion_id: message_result.discussion_id,
        })
    }

    pub async fn get_message_by_id(&self, message_id: i32) -> Result<MessageDto, AppError> {
        use crate::schema::message::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let message_result = message
            .find(message_id)
            .first::<Message>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        Ok(MessageDto {
            id: message_result.id,
            sender_id: message_result.sender_id,
            content: message_result.content,
            discussion_id: message_result.discussion_id,
        })
    }

    pub async fn get_all_messages(&self) -> Result<Vec<MessageDto>, AppError> {
        use crate::schema::message::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = message.load::<Message>(conn)?;

        let message_dtos = results
            .into_iter()
            .map(|m| MessageDto {
                id: m.id,
                sender_id: m.sender_id,
                content: m.content,
                discussion_id: m.discussion_id,
            })
            .collect();

        Ok(message_dtos)
    }

    pub async fn update_message(
        &self,
        message_id: i32,
        dto: UpdateMessageDto,
    ) -> Result<MessageDto, AppError> {
        use crate::schema::message::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if message exists
        let message_exists = message
            .find(message_id)
            .first::<Message>(conn)
            .optional()?
            .is_some();

        if !message_exists {
            return Err(AppError::NotFound("Message not found".to_string()));
        }

        let update_message = UpdateMessage {
            sender_id: dto.sender_id,
            content: dto.content,
            discussion_id: dto.discussion_id,
        };

        let updated_message = diesel::update(message.find(message_id))
            .set(&update_message)
            .get_result::<Message>(conn)?;

        Ok(MessageDto {
            id: updated_message.id,
            sender_id: updated_message.sender_id,
            content: updated_message.content,
            discussion_id: updated_message.discussion_id,
        })
    }

    pub async fn delete_message(&self, message_id: i32) -> Result<(), AppError> {
        use crate::schema::message::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if message exists
        let message_exists = message
            .find(message_id)
            .first::<Message>(conn)
            .optional()?
            .is_some();

        if !message_exists {
            return Err(AppError::NotFound("Message not found".to_string()));
        }

        diesel::delete(message.find(message_id)).execute(conn)?;

        Ok(())
    }

    pub async fn get_messages_by_discussion_id(
        &self,
        discussion_id_val: i32,
    ) -> Result<Vec<MessageDto>, AppError> {
        use crate::schema::message::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = message
            .filter(discussion_id.eq(discussion_id_val))
            .load::<Message>(conn)?;

        let message_dtos = results
            .into_iter()
            .map(|m| MessageDto {
                id: m.id,
                sender_id: m.sender_id,
                content: m.content,
                discussion_id: m.discussion_id,
            })
            .collect();

        Ok(message_dtos)
    }

    pub async fn get_messages_by_sender_id(
        &self,
        sender_id_val: i32,
    ) -> Result<Vec<MessageDto>, AppError> {
        use crate::schema::message::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = message
            .filter(sender_id.eq(sender_id_val))
            .load::<Message>(conn)?;

        let message_dtos = results
            .into_iter()
            .map(|m| MessageDto {
                id: m.id,
                sender_id: m.sender_id,
                content: m.content,
                discussion_id: m.discussion_id,
            })
            .collect();

        Ok(message_dtos)
    }
}

impl BaseService for MessageService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

