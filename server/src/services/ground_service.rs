use crate::dto::{CreateGroundDto, GroundDto, UpdateGroundDto};
use crate::errors::AppError;
use crate::models::{Ground, NewGround, UpdateGround};
use crate::schema::ground;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct GroundService {
    base: Service,
}

impl GroundService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_ground(&self, dto: CreateGroundDto) -> Result<GroundDto, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_ground = NewGround {
            name: dto.name,
            description: dto.description,
            folder: dto.folder,
            culture_type: dto.culture_type,
            location: dto.location,
            user_id: dto.user_id,
            pack: dto.pack,
        };

        let ground_result = diesel::insert_into(ground)
            .values(&new_ground)
            .get_result::<Ground>(conn)?;

        Ok(GroundDto {
            id: ground_result.id,
            name: ground_result.name,
            description: ground_result.description,
            folder: ground_result.folder,
            culture_type: ground_result.culture_type,
            location: ground_result.location,
            user_id: ground_result.user_id,
            pack: ground_result.pack,
        })
    }

    pub async fn get_ground_by_id(&self, ground_id: i32) -> Result<GroundDto, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let ground_result = ground
            .find(ground_id)
            .first::<Ground>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Ground not found".to_string()))?;

        Ok(GroundDto {
            id: ground_result.id,
            name: ground_result.name,
            description: ground_result.description,
            folder: ground_result.folder,
            culture_type: ground_result.culture_type,
            location: ground_result.location,
            user_id: ground_result.user_id,
            pack: ground_result.pack,
        })
    }

    pub async fn get_all_grounds(&self) -> Result<Vec<GroundDto>, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = ground.load::<Ground>(conn)?;

        let ground_dtos = results
            .into_iter()
            .map(|g| GroundDto {
                id: g.id,
                name: g.name,
                description: g.description,
                folder: g.folder,
                culture_type: g.culture_type,
                location: g.location,
                user_id: g.user_id,
                pack: g.pack,
            })
            .collect();

        Ok(ground_dtos)
    }

    pub async fn update_ground(
        &self,
        ground_id: i32,
        dto: UpdateGroundDto,
    ) -> Result<GroundDto, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if ground exists
        let ground_exists = ground
            .find(ground_id)
            .first::<Ground>(conn)
            .optional()?
            .is_some();

        if !ground_exists {
            return Err(AppError::NotFound("Ground not found".to_string()));
        }

        let update_ground = UpdateGround {
            name: dto.name,
            description: dto.description,
            folder: dto.folder,
            culture_type: dto.culture_type,
            location: dto.location,
            user_id: dto.user_id,
            pack: dto.pack,
        };

        let updated_ground = diesel::update(ground.find(ground_id))
            .set(&update_ground)
            .get_result::<Ground>(conn)?;

        Ok(GroundDto {
            id: updated_ground.id,
            name: updated_ground.name,
            description: updated_ground.description,
            folder: updated_ground.folder,
            culture_type: updated_ground.culture_type,
            location: updated_ground.location,
            user_id: updated_ground.user_id,
            pack: updated_ground.pack,
        })
    }

    pub async fn delete_ground(&self, ground_id: i32) -> Result<(), AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if ground exists
        let ground_exists = ground
            .find(ground_id)
            .first::<Ground>(conn)
            .optional()?
            .is_some();

        if !ground_exists {
            return Err(AppError::NotFound("Ground not found".to_string()));
        }

        diesel::delete(ground.find(ground_id)).execute(conn)?;

        Ok(())
    }

    pub async fn get_grounds_by_user_id(&self, user_id_val: i32) -> Result<Vec<GroundDto>, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = ground
            .filter(user_id.eq(user_id_val))
            .load::<Ground>(conn)?;

        let ground_dtos = results
            .into_iter()
            .map(|g| GroundDto {
                id: g.id,
                name: g.name,
                description: g.description,
                folder: g.folder,
                culture_type: g.culture_type,
                location: g.location,
                user_id: g.user_id,
                pack: g.pack,
            })
            .collect();

        Ok(ground_dtos)
    }

    pub async fn get_grounds_by_culture_type(&self, culture_type_id: i32) -> Result<Vec<GroundDto>, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = ground
            .filter(culture_type.eq(culture_type_id))
            .load::<Ground>(conn)?;

        let ground_dtos = results
            .into_iter()
            .map(|g| GroundDto {
                id: g.id,
                name: g.name,
                description: g.description,
                folder: g.folder,
                culture_type: g.culture_type,
                location: g.location,
                user_id: g.user_id,
                pack: g.pack,
            })
            .collect();

        Ok(ground_dtos)
    }

    pub async fn get_grounds_by_location(&self, location_id: i32) -> Result<Vec<GroundDto>, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = ground
            .filter(location.eq(location_id))
            .load::<Ground>(conn)?;

        let ground_dtos = results
            .into_iter()
            .map(|g| GroundDto {
                id: g.id,
                name: g.name,
                description: g.description,
                folder: g.folder,
                culture_type: g.culture_type,
                location: g.location,
                user_id: g.user_id,
                pack: g.pack,
            })
            .collect();

        Ok(ground_dtos)
    }

    pub async fn get_grounds_by_sensor_pack(&self, sensor_pack_id: &str) -> Result<Vec<GroundDto>, AppError> {
        use crate::schema::ground::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = ground
            .filter(pack.eq(sensor_pack_id))
            .load::<Ground>(conn)?;

        let ground_dtos = results
            .into_iter()
            .map(|g| GroundDto {
                id: g.id,
                name: g.name,
                description: g.description,
                folder: g.folder,
                culture_type: g.culture_type,
                location: g.location,
                user_id: g.user_id,
                pack: g.pack,
            })
            .collect();

        Ok(ground_dtos)
    }
}

impl BaseService for GroundService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

