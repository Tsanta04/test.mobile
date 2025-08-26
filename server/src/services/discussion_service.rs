use crate::error::AppError;
use crate::models::discussion::{Discussion, DiscussionUpdate, NewDiscussion};
use crate::models::message::{Message, NewMessage};
use crate::models::participant::NewParticipant;
use crate::schema::{discussion, message, participant};
use diesel::prelude::*;
use diesel::PgConnection;

pub struct DiscussionService;

impl DiscussionService {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Discussion>, AppError> {
        discussion::table
            .select(Discussion::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<Discussion, AppError> {
        discussion::table
            .find(id)
            .select(Discussion::as_select())
            .first(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_participant(user_id: i32, conn: &mut PgConnection) -> Result<Vec<Discussion>, AppError> {
        participant::table
            .filter(participant::participant_id.eq(user_id))
            .inner_join(discussion::table)
            .select(Discussion::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn get_messages(discussion_id: i32, conn: &mut PgConnection) -> Result<Vec<Message>, AppError> {
        message::table
            .filter(message::discussion_id.eq(discussion_id))
            .select(Message::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn create(
        new_discussion: NewDiscussion,
        participants: Vec<i32>,
        conn: &mut PgConnection,
    ) -> Result<Discussion, AppError> {
        conn.transaction(|conn| {
            // Create the discussion
            let discussion = diesel::insert_into(discussion::table)
                .values(&new_discussion)
                .returning(Discussion::as_returning())
                .get_result(conn)?;

            // Add participants
            for user_id in participants {
                let new_participant = NewParticipant {
                    discussion_id: Some(discussion.id),
                    participant_id: Some(user_id),
                };

                diesel::insert_into(participant::table)
                    .values(&new_participant)
                    .execute(conn)?;
            }

            Ok(discussion)
        })
    }

    pub fn add_message(
        discussion_id: i32,
        sender_id: i32,
        content: String,
        conn: &mut PgConnection,
    ) -> Result<Message, AppError> {
        let new_message = NewMessage {
            discussion_id: Some(discussion_id),
            sender_id: Some(sender_id),
            content: Some(content),
        };

        diesel::insert_into(message::table)
            .values(&new_message)
            .returning(Message::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn add_participant(
        discussion_id: i32,
        user_id: i32,
        conn: &mut PgConnection,
    ) -> Result<(), AppError> {
        let new_participant = NewParticipant {
            discussion_id: Some(discussion_id),
            participant_id: Some(user_id),
        };

        diesel::insert_into(participant::table)
            .values(&new_participant)
            .execute(conn)
            .map_err(AppError::from)?;

        Ok(())
    }

    pub fn update(
        id: i32,
        discussion_update: DiscussionUpdate,
        conn: &mut PgConnection,
    ) -> Result<Discussion, AppError> {
        diesel::update(discussion::table.find(id))
            .set(&discussion_update)
            .returning(Discussion::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, AppError> {
        conn.transaction(|conn| {
            // Delete all participants
            diesel::delete(participant::table.filter(participant::discussion_id.eq(id)))
                .execute(conn)?;

            // Delete all messages
            diesel::delete(message::table.filter(message::discussion_id.eq(id)))
                .execute(conn)?;

            // Delete the discussion
            diesel::delete(discussion::table.find(id)).execute(conn)
        })
        .map_err(AppError::from)
    }
}

