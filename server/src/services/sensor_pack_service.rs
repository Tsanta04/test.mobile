use crate::dto::{CreateSensorPackDto, SensorPackDto, UpdateSensorPackDto};
use crate::errors::AppError;
use crate::models::{NewSensorPack, SensorPack, UpdateSensorPack};
use crate::schema::sensor_pack;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct SensorPackService {
    base: Service,
}

impl SensorPackService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_sensor_pack(
        &self,
        dto: CreateSensorPackDto,
    ) -> Result<SensorPackDto, AppError> {
        use crate::schema::sensor_pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if sensor pack with same id exists
        let sensor_pack_exists = sensor_pack
            .find(&dto.id)
            .first::<SensorPack>(conn)
            .optional()?
            .is_some();

        if sensor_pack_exists {
            return Err(AppError::BadRequest(
                "Sensor pack with this ID already exists".to_string(),
            ));
        }

        let new_sensor_pack = NewSensorPack {
            id: dto.id,
            description: dto.description,
        };

        let sensor_pack_result = diesel::insert_into(sensor_pack)
            .values(&new_sensor_pack)
            .get_result::<SensorPack>(conn)?;

        Ok(SensorPackDto {
            id: sensor_pack_result.id,
            description: sensor_pack_result.description,
        })
    }

    pub async fn get_sensor_pack_by_id(&self, sensor_pack_id: &str) -> Result<SensorPackDto, AppError> {
        use crate::schema::sensor_pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let sensor_pack_result = sensor_pack
            .find(sensor_pack_id)
            .first::<SensorPack>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Sensor pack not found".to_string()))?;

        Ok(SensorPackDto {
            id: sensor_pack_result.id,
            description: sensor_pack_result.description,
        })
    }

    pub async fn get_all_sensor_packs(&self) -> Result<Vec<SensorPackDto>, AppError> {
        use crate::schema::sensor_pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = sensor_pack.load::<SensorPack>(conn)?;

        let sensor_pack_dtos = results
            .into_iter()
            .map(|sp| SensorPackDto {
                id: sp.id,
                description: sp.description,
            })
            .collect();

        Ok(sensor_pack_dtos)
    }

    pub async fn update_sensor_pack(
        &self,
        sensor_pack_id: &str,
        dto: UpdateSensorPackDto,
    ) -> Result<SensorPackDto, AppError> {
        use crate::schema::sensor_pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if sensor pack exists
        let sensor_pack_exists = sensor_pack
            .find(sensor_pack_id)
            .first::<SensorPack>(conn)
            .optional()?
            .is_some();

        if !sensor_pack_exists {
            return Err(AppError::NotFound("Sensor pack not found".to_string()));
        }

        let update_sensor_pack = UpdateSensorPack {
            description: dto.description,
        };

        let updated_sensor_pack = diesel::update(sensor_pack.find(sensor_pack_id))
            .set(&update_sensor_pack)
            .get_result::<SensorPack>(conn)?;

        Ok(SensorPackDto {
            id: updated_sensor_pack.id,
            description: updated_sensor_pack.description,
        })
    }

    pub async fn delete_sensor_pack(&self, sensor_pack_id: &str) -> Result<(), AppError> {
        use crate::schema::sensor_pack::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if sensor pack exists
        let sensor_pack_exists = sensor_pack
            .find(sensor_pack_id)
            .first::<SensorPack>(conn)
            .optional()?
            .is_some();

        if !sensor_pack_exists {
            return Err(AppError::NotFound("Sensor pack not found".to_string()));
        }

        diesel::delete(sensor_pack.find(sensor_pack_id)).execute(conn)?;

        Ok(())
    }
}

impl BaseService for SensorPackService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

