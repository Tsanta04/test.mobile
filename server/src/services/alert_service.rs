use crate::dto::{AlertDto, CreateAlertDto, UpdateAlertDto};
use crate::errors::AppError;
use crate::models::{Alert, NewAlert, UpdateAlert};
use crate::models::types::{AlertType, LevelType};
use crate::schema::alert;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct AlertService {
    base: Service,
}

impl AlertService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_alert(&self, dto: CreateAlertDto) -> Result<AlertDto, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_alert = NewAlert {
            date: dto.date,
            title: dto.title,
            description: dto.description,
            type_: dto.type_,
            level: dto.level,
            recommandation: dto.recommandation,
            isseen: dto.isseen,
            state_id: dto.state_id,
        };

        let alert_result = diesel::insert_into(alert)
            .values(&new_alert)
            .get_result::<Alert>(conn)?;

        Ok(AlertDto {
            id: alert_result.id,
            date: alert_result.date,
            title: alert_result.title,
            description: alert_result.description,
            type_: alert_result.type_,
            level: alert_result.level,
            recommandation: alert_result.recommandation,
            isseen: alert_result.isseen,
            state_id: alert_result.state_id,
        })
    }

    pub async fn get_alert_by_id(&self, alert_id: i32) -> Result<AlertDto, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let alert_result = alert
            .find(alert_id)
            .first::<Alert>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Alert not found".to_string()))?;

        Ok(AlertDto {
            id: alert_result.id,
            date: alert_result.date,
            title: alert_result.title,
            description: alert_result.description,
            type_: alert_result.type_,
            level: alert_result.level,
            recommandation: alert_result.recommandation,
            isseen: alert_result.isseen,
            state_id: alert_result.state_id,
        })
    }

    pub async fn get_all_alerts(&self) -> Result<Vec<AlertDto>, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = alert.load::<Alert>(conn)?;

        let alert_dtos = results
            .into_iter()
            .map(|a| AlertDto {
                id: a.id,
                date: a.date,
                title: a.title,
                description: a.description,
                type_: a.type_,
                level: a.level,
                recommandation: a.recommandation,
                isseen: a.isseen,
                state_id: a.state_id,
            })
            .collect();

        Ok(alert_dtos)
    }

    pub async fn update_alert(
        &self,
        alert_id: i32,
        dto: UpdateAlertDto,
    ) -> Result<AlertDto, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if alert exists
        let alert_exists = alert
            .find(alert_id)
            .first::<Alert>(conn)
            .optional()?
            .is_some();

        if !alert_exists {
            return Err(AppError::NotFound("Alert not found".to_string()));
        }

        let update_alert = UpdateAlert {
            date: dto.date,
            title: dto.title,
            description: dto.description,
            type_: dto.type_,
            level: dto.level,
            recommandation: dto.recommandation,
            isseen: dto.isseen,
            state_id: dto.state_id,
        };

        let updated_alert = diesel::update(alert.find(alert_id))
            .set(&update_alert)
            .get_result::<Alert>(conn)?;

        Ok(AlertDto {
            id: updated_alert.id,
            date: updated_alert.date,
            title: updated_alert.title,
            description: updated_alert.description,
            type_: updated_alert.type_,
            level: updated_alert.level,
            recommandation: updated_alert.recommandation,
            isseen: updated_alert.isseen,
            state_id: updated_alert.state_id,
        })
    }

    pub async fn delete_alert(&self, alert_id: i32) -> Result<(), AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if alert exists
        let alert_exists = alert
            .find(alert_id)
            .first::<Alert>(conn)
            .optional()?
            .is_some();

        if !alert_exists {
            return Err(AppError::NotFound("Alert not found".to_string()));
        }

        diesel::delete(alert.find(alert_id)).execute(conn)?;

        Ok(())
    }

    pub async fn get_alerts_by_state_id(&self, state_id_val: i32) -> Result<Vec<AlertDto>, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = alert
            .filter(state_id.eq(state_id_val))
            .load::<Alert>(conn)?;

        let alert_dtos = results
            .into_iter()
            .map(|a| AlertDto {
                id: a.id,
                date: a.date,
                title: a.title,
                description: a.description,
                type_: a.type_,
                level: a.level,
                recommandation: a.recommandation,
                isseen: a.isseen,
                state_id: a.state_id,
            })
            .collect();

        Ok(alert_dtos)
    }

    pub async fn get_alerts_by_type(&self, alert_type: AlertType) -> Result<Vec<AlertDto>, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = alert
            .filter(type_.eq(alert_type))
            .load::<Alert>(conn)?;

        let alert_dtos = results
            .into_iter()
            .map(|a| AlertDto {
                id: a.id,
                date: a.date,
                title: a.title,
                description: a.description,
                type_: a.type_,
                level: a.level,
                recommandation: a.recommandation,
                isseen: a.isseen,
                state_id: a.state_id,
            })
            .collect();

        Ok(alert_dtos)
    }

    pub async fn get_alerts_by_level(&self, level_val: LevelType) -> Result<Vec<AlertDto>, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = alert
            .filter(level.eq(level_val))
            .load::<Alert>(conn)?;

        let alert_dtos = results
            .into_iter()
            .map(|a| AlertDto {
                id: a.id,
                date: a.date,
                title: a.title,
                description: a.description,
                type_: a.type_,
                level: a.level,
                recommandation: a.recommandation,
                isseen: a.isseen,
                state_id: a.state_id,
            })
            .collect();

        Ok(alert_dtos)
    }

    pub async fn get_unseen_alerts(&self) -> Result<Vec<AlertDto>, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = alert
            .filter(isseen.eq(false))
            .load::<Alert>(conn)?;

        let alert_dtos = results
            .into_iter()
            .map(|a| AlertDto {
                id: a.id,
                date: a.date,
                title: a.title,
                description: a.description,
                type_: a.type_,
                level: a.level,
                recommandation: a.recommandation,
                isseen: a.isseen,
                state_id: a.state_id,
            })
            .collect();

        Ok(alert_dtos)
    }

    pub async fn mark_alert_as_seen(&self, alert_id: i32) -> Result<AlertDto, AppError> {
        use crate::schema::alert::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if alert exists
        let alert_exists = alert
            .find(alert_id)
            .first::<Alert>(conn)
            .optional()?
            .is_some();

        if !alert_exists {
            return Err(AppError::NotFound("Alert not found".to_string()));
        }

        let updated_alert = diesel::update(alert.find(alert_id))
            .set(isseen.eq(true))
            .get_result::<Alert>(conn)?;

        Ok(AlertDto {
            id: updated_alert.id,
            date: updated_alert.date,
            title: updated_alert.title,
            description: updated_alert.description,
            type_: updated_alert.type_,
            level: updated_alert.level,
            recommandation: updated_alert.recommandation,
            isseen: updated_alert.isseen,
            state_id: updated_alert.state_id,
        })
    }
}

impl BaseService for AlertService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

