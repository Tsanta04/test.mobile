use crate::schema::participant;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Participant {
    pub id: i32,
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = participant)]
pub struct NewParticipant {
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = participant)]
pub struct UpdateParticipant {
    pub discussion_id: Option<i32>,
    pub participant_id: Option<i32>,
}

