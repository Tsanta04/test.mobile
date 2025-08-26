use crate::dto::{CreatePlanningDto, PlanningDto, UpdatePlanningDto};
use crate::errors::AppError;
use crate::models::{NewPlanning, Planning, UpdatePlanning};
use crate::schema::planning;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct PlanningService {
    base: Service,
}

impl PlanningService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_planning(&self, dto: CreatePlanningDto) -> Result<PlanningDto, AppError> {
        use crate::schema::planning::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_planning = NewPlanning {
            date: dto.date,
            title: dto.title,
            description: dto.description,
            start_date: dto.start_date,
            end_date: dto.end_date,
            ground: dto.ground,
        };

        let planning_result = diesel::insert_into(planning)
            .values(&new_planning)
            .get_result::<Planning>(conn)?;

        Ok(PlanningDto {
            id: planning_result.id,
            date: planning_result.date,
            title: planning_result.title,
            description: planning_result.description,
            start_date: planning_result.start_date,
            end_date: planning_result.end_date,
            ground: planning_result.ground,
        })
    }

    pub async fn get_planning_by_id(&self, planning_id: i32) -> Result<PlanningDto, AppError> {
        use crate::schema::planning::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let planning_result = planning
            .find(planning_id)
            .first::<Planning>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Planning not found".to_string()))?;

        Ok(PlanningDto {
            id: planning_result.id,
            date: planning_result.date,
            title: planning_result.title,
            description: planning_result.description,
            start_date: planning_result.start_date,
            end_date: planning_result.end_date,
            ground: planning_result.ground,
        })
    }

    pub async fn get_all_plannings(&self) -> Result<Vec<PlanningDto>, AppError> {
        use crate::schema::planning::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = planning.load::<Planning>(conn)?;

        let planning_dtos = results
            .into_iter()
            .map(|p| PlanningDto {
                id: p.id,
                date: p.date,
                title: p.title,
                description: p.description,
                start_date: p.start_date,
                end_date: p.end_date,
                ground: p.ground,
            })
            .collect();

        Ok(planning_dtos)
    }

    pub async fn update_planning(
        &self,
        planning_id: i32,
        dto: UpdatePlanningDto,
    ) -> Result<PlanningDto, AppError> {
        use crate::schema::planning::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if planning exists
        let planning_exists = planning
            .find(planning_id)
            .first::<Planning>(conn)
            .optional()?
            .is_some();

        if !planning_exists {
            return Err(AppError::NotFound("Planning not found".to_string()));
        }

        let update_planning = UpdatePlanning {
            date: dto.date,
            title: dto.title,
            description: dto.description,
            start_date: dto.start_date,
            end_date: dto.end_date,
            ground: dto.ground,
        };

        let updated_planning = diesel::update(planning.find(planning_id))
            .set(&update_planning)
            .get_result::<Planning>(conn)?;

        Ok(PlanningDto {
            id: updated_planning.id,
            date: updated_planning.date,
            title: updated_planning.title,
            description: updated_planning.description,
            start_date: updated_planning.start_date,
            end_date: updated_planning.end_date,
            ground: updated_planning.ground,
        })
    }

    pub async fn delete_planning(&self, planning_id: i32) -> Result<(), AppError> {
        use crate::schema::planning::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if planning exists
        let planning_exists = planning
            .find(planning_id)
            .first::<Planning>(conn)
            .optional()?
            .is_some();

        if !planning_exists {
            return Err(AppError::NotFound("Planning not found".to_string()));
        }

        diesel::delete(planning.find(planning_id)).execute(conn)?;

        Ok(())
    }

    pub async fn get_plannings_by_ground_id(&self, ground_id: i32) -> Result<Vec<PlanningDto>, AppError> {
        use crate::schema::planning::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = planning
            .filter(ground.eq(ground_id))
            .load::<Planning>(conn)?;

        let planning_dtos = results
            .into_iter()
            .map(|p| PlanningDto {
                id: p.id,
                date: p.date,
                title: p.title,
                description: p.description,
                start_date: p.start_date,
                end_date: p.end_date,
                ground: p.ground,
            })
            .collect();

        Ok(planning_dtos)
    }

    pub async fn get_current_plannings(&self) -> Result<Vec<PlanningDto>, AppError> {
        use crate::schema::planning::dsl::*;
        use diesel::dsl::now;

        let conn = &mut self.base.get_pool().get()?;

        let results = planning
            .filter(start_date.le(now))
            .filter(end_date.ge(now))
            .load::<Planning>(conn)?;

        let planning_dtos = results
            .into_iter()
            .map(|p| PlanningDto {
                id: p.id,
                date: p.date,
                title: p.title,
                description: p.description,
                start_date: p.start_date,
                end_date: p.end_date,
                ground: p.ground,
            })
            .collect();

        Ok(planning_dtos)
    }

    pub async fn get_upcoming_plannings(&self) -> Result<Vec<PlanningDto>, AppError> {
        use crate::schema::planning::dsl::*;
        use diesel::dsl::now;

        let conn = &mut self.base.get_pool().get()?;

        let results = planning
            .filter(start_date.gt(now))
            .order(start_date.asc())
            .load::<Planning>(conn)?;

        let planning_dtos = results
            .into_iter()
            .map(|p| PlanningDto {
                id: p.id,
                date: p.date,
                title: p.title,
                description: p.description,
                start_date: p.start_date,
                end_date: p.end_date,
                ground: p.ground,
            })
            .collect();

        Ok(planning_dtos)
    }
}

impl BaseService for PlanningService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

