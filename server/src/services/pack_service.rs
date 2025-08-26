use crate::dto::{CreatePackDto, PackDto, UpdatePackDto};
use crate::errors::AppError;
use crate::models::{NewPack, Pack, UpdatePack};
use crate::schema::pack;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct PackService {
    base: Service,
}

impl PackService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_pack(&self, dto: CreatePackDto) -> Result<PackDto, AppError> {
        use crate::schema::pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_pack = NewPack {
            pack_id: dto.pack_id,
            sensor_id: dto.sensor_id,
        };

        let pack_result = diesel::insert_into(pack)
            .values(&new_pack)
            .get_result::<Pack>(conn)?;

        Ok(PackDto {
            id: pack_result.id,
            pack_id: pack_result.pack_id,
            sensor_id: pack_result.sensor_id,
        })
    }

    pub async fn get_pack_by_id(&self, pack_id_val: i32) -> Result<PackDto, AppError> {
        use crate::schema::pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let pack_result = pack
            .find(pack_id_val)
            .first::<Pack>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Pack not found".to_string()))?;

        Ok(PackDto {
            id: pack_result.id,
            pack_id: pack_result.pack_id,
            sensor_id: pack_result.sensor_id,
        })
    }

    pub async fn get_all_packs(&self) -> Result<Vec<PackDto>, AppError> {
        use crate::schema::pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = pack.load::<Pack>(conn)?;

        let pack_dtos = results
            .into_iter()
            .map(|p| PackDto {
                id: p.id,
                pack_id: p.pack_id,
                sensor_id: p.sensor_id,
            })
            .collect();

        Ok(pack_dtos)
    }

    pub async fn update_pack(
        &self,
        pack_id_val: i32,
        dto: UpdatePackDto,
    ) -> Result<PackDto, AppError> {
        use crate::schema::pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if pack exists
        let pack_exists = pack
            .find(pack_id_val)
            .first::<Pack>(conn)
            .optional()?
            .is_some();

        if !pack_exists {
            return Err(AppError::NotFound("Pack not found".to_string()));
        }

        let update_pack = UpdatePack {
            pack_id: dto.pack_id,
            sensor_id: dto.sensor_id,
        };

        let updated_pack = diesel::update(pack.find(pack_id_val))
            .set(&update_pack)
            .get_result::<Pack>(conn)?;

        Ok(PackDto {
            id: updated_pack.id,
            pack_id: updated_pack.pack_id,
            sensor_id: updated_pack.sensor_id,
        })
    }

    pub async fn delete_pack(&self, pack_id_val: i32) -> Result<(), AppError> {
        use crate::schema::pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if pack exists
        let pack_exists = pack
            .find(pack_id_val)
            .first::<Pack>(conn)
            .optional()?
            .is_some();

        if !pack_exists {
            return Err(AppError::NotFound("Pack not found".to_string()));
        }

        diesel::delete(pack.find(pack_id_val)).execute(conn)?;

        Ok(())
    }

    pub async fn get_packs_by_sensor_pack_id(&self, sensor_pack_id: &str) -> Result<Vec<PackDto>, AppError> {
        use crate::schema::pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = pack
            .filter(pack_id.eq(sensor_pack_id))
            .load::<Pack>(conn)?;

        let pack_dtos = results
            .into_iter()
            .map(|p| PackDto {
                id: p.id,
                pack_id: p.pack_id,
                sensor_id: p.sensor_id,
            })
            .collect();

        Ok(pack_dtos)
    }

    pub async fn get_packs_by_sensor_id(&self, sensor_id_val: i32) -> Result<Vec<PackDto>, AppError> {
        use crate::schema::pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = pack
            .filter(sensor_id.eq(sensor_id_val))
            .load::<Pack>(conn)?;

        let pack_dtos = results
            .into_iter()
            .map(|p| PackDto {
                id: p.id,
                pack_id: p.pack_id,
                sensor_id: p.sensor_id,
            })
            .collect();

        Ok(pack_dtos)
    }
}

impl BaseService for PackService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

