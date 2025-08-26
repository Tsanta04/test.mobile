use crate::dto::{CreateCultureTypeDto, CultureTypeDto, UpdateCultureTypeDto};
use crate::errors::AppError;
use crate::models::{CultureType, NewCultureType, UpdateCultureType};
use crate::schema::culture_type;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct CultureTypeService {
    base: Service,
}

impl CultureTypeService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_culture_type(
        &self,
        dto: CreateCultureTypeDto,
    ) -> Result<CultureTypeDto, AppError> {
        use crate::schema::culture_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_culture_type = NewCultureType { type_: dto.type_ };

        let culture_type_result = diesel::insert_into(culture_type)
            .values(&new_culture_type)
            .get_result::<CultureType>(conn)?;

        Ok(CultureTypeDto {
            id: culture_type_result.id,
            type_: culture_type_result.type_,
        })
    }

    pub async fn get_culture_type_by_id(&self, culture_type_id: i32) -> Result<CultureTypeDto, AppError> {
        use crate::schema::culture_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let culture_type_result = culture_type
            .find(culture_type_id)
            .first::<CultureType>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Culture type not found".to_string()))?;

        Ok(CultureTypeDto {
            id: culture_type_result.id,
            type_: culture_type_result.type_,
        })
    }

    pub async fn get_all_culture_types(&self) -> Result<Vec<CultureTypeDto>, AppError> {
        use crate::schema::culture_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = culture_type.load::<CultureType>(conn)?;

        let culture_type_dtos = results
            .into_iter()
            .map(|ct| CultureTypeDto {
                id: ct.id,
                type_: ct.type_,
            })
            .collect();

        Ok(culture_type_dtos)
    }

    pub async fn update_culture_type(
        &self,
        culture_type_id: i32,
        dto: UpdateCultureTypeDto,
    ) -> Result<CultureTypeDto, AppError> {
        use crate::schema::culture_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if culture type exists
        let culture_type_exists = culture_type
            .find(culture_type_id)
            .first::<CultureType>(conn)
            .optional()?
            .is_some();

        if !culture_type_exists {
            return Err(AppError::NotFound("Culture type not found".to_string()));
        }

        let update_culture_type = UpdateCultureType { type_: dto.type_ };

        let updated_culture_type = diesel::update(culture_type.find(culture_type_id))
            .set(&update_culture_type)
            .get_result::<CultureType>(conn)?;

        Ok(CultureTypeDto {
            id: updated_culture_type.id,
            type_: updated_culture_type.type_,
        })
    }

    pub async fn delete_culture_type(&self, culture_type_id: i32) -> Result<(), AppError> {
        use crate::schema::culture_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if culture type exists
        let culture_type_exists = culture_type
            .find(culture_type_id)
            .first::<CultureType>(conn)
            .optional()?
            .is_some();

        if !culture_type_exists {
            return Err(AppError::NotFound("Culture type not found".to_string()));
        }

        diesel::delete(culture_type.find(culture_type_id)).execute(conn)?;

        Ok(())
    }
}

impl BaseService for CultureTypeService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

