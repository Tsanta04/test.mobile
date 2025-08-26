use crate::dto::{CreateSensorDto, SensorDto, UpdateSensorDto};
use crate::errors::AppError;
use crate::models::{NewSensor, Sensor, UpdateSensor};
use crate::schema::sensor;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct SensorService {
    base: Service,
}

impl SensorService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_sensor(&self, dto: CreateSensorDto) -> Result<SensorDto, AppError> {
        use crate::schema::sensor::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_sensor = NewSensor {
            description: dto.description,
            issue_date: dto.issue_date,
            sensor_type: dto.sensor_type,
        };

        let sensor_result = diesel::insert_into(sensor)
            .values(&new_sensor)
            .get_result::<Sensor>(conn)?;

        Ok(SensorDto {
            id: sensor_result.id,
            description: sensor_result.description,
            issue_date: sensor_result.issue_date,
            sensor_type: sensor_result.sensor_type,
        })
    }

    pub async fn get_sensor_by_id(&self, sensor_id: i32) -> Result<SensorDto, AppError> {
        use crate::schema::sensor::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let sensor_result = sensor
            .find(sensor_id)
            .first::<Sensor>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Sensor not found".to_string()))?;

        Ok(SensorDto {
            id: sensor_result.id,
            description: sensor_result.description,
            issue_date: sensor_result.issue_date,
            sensor_type: sensor_result.sensor_type,
        })
    }

    pub async fn get_all_sensors(&self) -> Result<Vec<SensorDto>, AppError> {
        use crate::schema::sensor::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = sensor.load::<Sensor>(conn)?;

        let sensor_dtos = results
            .into_iter()
            .map(|s| SensorDto {
                id: s.id,
                description: s.description,
                issue_date: s.issue_date,
                sensor_type: s.sensor_type,
            })
            .collect();

        Ok(sensor_dtos)
    }

    pub async fn update_sensor(
        &self,
        sensor_id: i32,
        dto: UpdateSensorDto,
    ) -> Result<SensorDto, AppError> {
        use crate::schema::sensor::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if sensor exists
        let sensor_exists = sensor
            .find(sensor_id)
            .first::<Sensor>(conn)
            .optional()?
            .is_some();

        if !sensor_exists {
            return Err(AppError::NotFound("Sensor not found".to_string()));
        }

        let update_sensor = UpdateSensor {
            description: dto.description,
            issue_date: dto.issue_date,
            sensor_type: dto.sensor_type,
        };

        let updated_sensor = diesel::update(sensor.find(sensor_id))
            .set(&update_sensor)
            .get_result::<Sensor>(conn)?;

        Ok(SensorDto {
            id: updated_sensor.id,
            description: updated_sensor.description,
            issue_date: updated_sensor.issue_date,
            sensor_type: updated_sensor.sensor_type,
        })
    }

    pub async fn delete_sensor(&self, sensor_id: i32) -> Result<(), AppError> {
        use crate::schema::sensor::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if sensor exists
        let sensor_exists = sensor
            .find(sensor_id)
            .first::<Sensor>(conn)
            .optional()?
            .is_some();

        if !sensor_exists {
            return Err(AppError::NotFound("Sensor not found".to_string()));
        }

        diesel::delete(sensor.find(sensor_id)).execute(conn)?;

        Ok(())
    }

    pub async fn get_sensors_by_type(&self, sensor_type_id: i32) -> Result<Vec<SensorDto>, AppError> {
        use crate::schema::sensor::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = sensor
            .filter(sensor_type.eq(sensor_type_id))
            .load::<Sensor>(conn)?;

        let sensor_dtos = results
            .into_iter()
            .map(|s| SensorDto {
                id: s.id,
                description: s.description,
                issue_date: s.issue_date,
                sensor_type: s.sensor_type,
            })
            .collect();

        Ok(sensor_dtos)
    }
}

impl BaseService for SensorService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

