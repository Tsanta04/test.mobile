use crate::error::AppError;
use crate::models::state::{NewState, State, StateUpdate};
use crate::schema::state;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct StateService;

impl StateService {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<State>, AppError> {
        state::table
            .select(State::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<State, AppError> {
        state::table
            .find(id)
            .select(State::as_select())
            .first(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_pack(pack_id: &str, conn: &mut PgConnection) -> Result<Vec<State>, AppError> {
        state::table
            .filter(state::pack_id.eq(pack_id))
            .select(State::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_latest_by_pack(pack_id: &str, conn: &mut PgConnection) -> Result<State, AppError> {
        state::table
            .filter(state::pack_id.eq(pack_id))
            .order(state::date.desc())
            .select(State::as_select())
            .first(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_date_range(
        pack_id: &str,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
        conn: &mut PgConnection,
    ) -> Result<Vec<State>, AppError> {
        state::table
            .filter(state::pack_id.eq(pack_id))
            .filter(state::date.ge(start_date))
            .filter(state::date.le(end_date))
            .select(State::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn create(new_state: NewState, conn: &mut PgConnection) -> Result<State, AppError> {
        diesel::insert_into(state::table)
            .values(&new_state)
            .returning(State::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn update(id: i32, state_update: StateUpdate, conn: &mut PgConnection) -> Result<State, AppError> {
        diesel::update(state::table.find(id))
            .set(&state_update)
            .returning(State::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, AppError> {
        diesel::delete(state::table.find(id))
            .execute(conn)
            .map_err(AppError::from)
    }
}

