use crate::schema::participant;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = participant)]
#[diesel(belongs_to(crate::models::user::User, foreign_key = participant_id))]
#[diesel(belongs_to(crate::models::discussion::Discussion, foreign_key = discussion_id))]
pub struct Participant {
    pub id: i32,
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = participant)]
pub struct NewParticipant {
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

