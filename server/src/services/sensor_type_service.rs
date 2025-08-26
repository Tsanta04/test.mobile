use crate::dto::{CreateSensorTypeDto, SensorTypeDto, UpdateSensorTypeDto};
use crate::errors::AppError;
use crate::models::{NewSensorType, SensorType, UpdateSensorType};
use crate::schema::sensor_type;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct SensorTypeService {
    base: Service,
}

impl SensorTypeService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_sensor_type(
        &self,
        dto: CreateSensorTypeDto,
    ) -> Result<SensorTypeDto, AppError> {
        use crate::schema::sensor_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_sensor_type = NewSensorType { type_: dto.type_ };

        let sensor_type_result = diesel::insert_into(sensor_type)
            .values(&new_sensor_type)
            .get_result::<SensorType>(conn)?;

        Ok(SensorTypeDto {
            id: sensor_type_result.id,
            type_: sensor_type_result.type_,
        })
    }

    pub async fn get_sensor_type_by_id(&self, sensor_type_id: i32) -> Result<SensorTypeDto, AppError> {
        use crate::schema::sensor_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let sensor_type_result = sensor_type
            .find(sensor_type_id)
            .first::<SensorType>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Sensor type not found".to_string()))?;

        Ok(SensorTypeDto {
            id: sensor_type_result.id,
            type_: sensor_type_result.type_,
        })
    }

    pub async fn get_all_sensor_types(&self) -> Result<Vec<SensorTypeDto>, AppError> {
        use crate::schema::sensor_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = sensor_type.load::<SensorType>(conn)?;

        let sensor_type_dtos = results
            .into_iter()
            .map(|st| SensorTypeDto {
                id: st.id,
                type_: st.type_,
            })
            .collect();

        Ok(sensor_type_dtos)
    }

    pub async fn update_sensor_type(
        &self,
        sensor_type_id: i32,
        dto: UpdateSensorTypeDto,
    ) -> Result<SensorTypeDto, AppError> {
        use crate::schema::sensor_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if sensor type exists
        let sensor_type_exists = sensor_type
            .find(sensor_type_id)
            .first::<SensorType>(conn)
            .optional()?
            .is_some();

        if !sensor_type_exists {
            return Err(AppError::NotFound("Sensor type not found".to_string()));
        }

        let update_sensor_type = UpdateSensorType { type_: dto.type_ };

        let updated_sensor_type = diesel::update(sensor_type.find(sensor_type_id))
            .set(&update_sensor_type)
            .get_result::<SensorType>(conn)?;

        Ok(SensorTypeDto {
            id: updated_sensor_type.id,
            type_: updated_sensor_type.type_,
        })
    }

    pub async fn delete_sensor_type(&self, sensor_type_id: i32) -> Result<(), AppError> {
        use crate::schema::sensor_type::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if sensor type exists
        let sensor_type_exists = sensor_type
            .find(sensor_type_id)
            .first::<SensorType>(conn)
            .optional()?
            .is_some();

        if !sensor_type_exists {
            return Err(AppError::NotFound("Sensor type not found".to_string()));
        }

        diesel::delete(sensor_type.find(sensor_type_id)).execute(conn)?;

        Ok(())
    }
}

impl BaseService for SensorTypeService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

