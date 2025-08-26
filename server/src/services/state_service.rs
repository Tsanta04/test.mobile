use crate::dto::{CreateStateDto, StateDto, UpdateStateDto};
use crate::errors::AppError;
use crate::models::{NewState, State, UpdateState};
use crate::schema::state;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct StateService {
    base: Service,
}

impl StateService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_state(&self, dto: CreateStateDto) -> Result<StateDto, AppError> {
        use crate::schema::state::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_state = NewState {
            date: dto.date,
            temperature: dto.temperature,
            health: dto.health,
            production_progress: dto.production_progress,
            humidity: dto.humidity,
            fertility: dto.fertility,
            rentability: dto.rentability,
            pack_id: dto.pack_id,
        };

        let state_result = diesel::insert_into(state)
            .values(&new_state)
            .get_result::<State>(conn)?;

        Ok(StateDto {
            id: state_result.id,
            date: state_result.date,
            temperature: state_result.temperature,
            health: state_result.health,
            production_progress: state_result.production_progress,
            humidity: state_result.humidity,
            fertility: state_result.fertility,
            rentability: state_result.rentability,
            pack_id: state_result.pack_id,
        })
    }

    pub async fn get_state_by_id(&self, state_id: i32) -> Result<StateDto, AppError> {
        use crate::schema::state::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let state_result = state
            .find(state_id)
            .first::<State>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("State not found".to_string()))?;

        Ok(StateDto {
            id: state_result.id,
            date: state_result.date,
            temperature: state_result.temperature,
            health: state_result.health,
            production_progress: state_result.production_progress,
            humidity: state_result.humidity,
            fertility: state_result.fertility,
            rentability: state_result.rentability,
            pack_id: state_result.pack_id,
        })
    }

    pub async fn get_all_states(&self) -> Result<Vec<StateDto>, AppError> {
        use crate::schema::state::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = state.load::<State>(conn)?;

        let state_dtos = results
            .into_iter()
            .map(|s| StateDto {
                id: s.id,
                date: s.date,
                temperature: s.temperature,
                health: s.health,
                production_progress: s.production_progress,
                humidity: s.humidity,
                fertility: s.fertility,
                rentability: s.rentability,
                pack_id: s.pack_id,
            })
            .collect();

        Ok(state_dtos)
    }

    pub async fn update_state(
        &self,
        state_id: i32,
        dto: UpdateStateDto,
    ) -> Result<StateDto, AppError> {
        use crate::schema::state::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if state exists
        let state_exists = state
            .find(state_id)
            .first::<State>(conn)
            .optional()?
            .is_some();

        if !state_exists {
            return Err(AppError::NotFound("State not found".to_string()));
        }

        let update_state = UpdateState {
            date: dto.date,
            temperature: dto.temperature,
            health: dto.health,
            production_progress: dto.production_progress,
            humidity: dto.humidity,
            fertility: dto.fertility,
            rentability: dto.rentability,
            pack_id: dto.pack_id,
        };

        let updated_state = diesel::update(state.find(state_id))
            .set(&update_state)
            .get_result::<State>(conn)?;

        Ok(StateDto {
            id: updated_state.id,
            date: updated_state.date,
            temperature: updated_state.temperature,
            health: updated_state.health,
            production_progress: updated_state.production_progress,
            humidity: updated_state.humidity,
            fertility: updated_state.fertility,
            rentability: updated_state.rentability,
            pack_id: updated_state.pack_id,
        })
    }

    pub async fn delete_state(&self, state_id: i32) -> Result<(), AppError> {
        use crate::schema::state::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if state exists
        let state_exists = state
            .find(state_id)
            .first::<State>(conn)
            .optional()?
            .is_some();

        if !state_exists {
            return Err(AppError::NotFound("State not found".to_string()));
        }

        diesel::delete(state.find(state_id)).execute(conn)?;

        Ok(())
    }

    pub async fn get_states_by_pack_id(&self, sensor_pack_id: &str) -> Result<Vec<StateDto>, AppError> {
        use crate::schema::state::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = state
            .filter(pack_id.eq(sensor_pack_id))
            .load::<State>(conn)?;

        let state_dtos = results
            .into_iter()
            .map(|s| StateDto {
                id: s.id,
                date: s.date,
                temperature: s.temperature,
                health: s.health,
                production_progress: s.production_progress,
                humidity: s.humidity,
                fertility: s.fertility,
                rentability: s.rentability,
                pack_id: s.pack_id,
            })
            .collect();

        Ok(state_dtos)
    }

    pub async fn get_latest_state_by_pack_id(&self, sensor_pack_id: &str) -> Result<StateDto, AppError> {
        use crate::schema::state::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let state_result = state
            .filter(pack_id.eq(sensor_pack_id))
            .order(date.desc())
            .first::<State>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("No state found for this pack".to_string()))?;

        Ok(StateDto {
            id: state_result.id,
            date: state_result.date,
            temperature: state_result.temperature,
            health: state_result.health,
            production_progress: state_result.production_progress,
            humidity: state_result.humidity,
            fertility: state_result.fertility,
            rentability: state_result.rentability,
            pack_id: state_result.pack_id,
        })
    }
}

impl BaseService for StateService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

