use crate::dto::sensor_type::{CreateSensorTypeDto, UpdateSensorTypeDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::sensor_type::{SensorType, NewSensorType, SensorTypeUpdate};
use sqlx::PgPool;

pub struct SensorTypeService;

impl SensorTypeService {
    pub async fn create_sensor_type(
        pool: &PgPool,
        sensor_type_dto: CreateSensorTypeDto,
    ) -> Result<SensorType, AppError> {
        // Create a new sensor type
        let new_sensor_type = NewSensorType {
            type_: sensor_type_dto.type_,
        };

        // Insert the sensor type into the database
        let sensor_type = sqlx::query!(
            r#"
            INSERT INTO Sensor_type (type)
            VALUES ($1)
            RETURNING id, type as "type_"
            "#,
            new_sensor_type.type_
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(SensorType {
            id: sensor_type.id,
            type_: sensor_type.type_,
        })
    }

    pub async fn get_sensor_type_by_id(pool: &PgPool, id: i32) -> Result<SensorType, AppError> {
        let sensor_type = sqlx::query!(
            r#"
            SELECT id, type as "type_"
            FROM Sensor_type
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(SensorType {
            id: sensor_type.id,
            type_: sensor_type.type_,
        })
    }

    pub async fn get_all_sensor_types(pool: &PgPool) -> Result<Vec<SensorType>, AppError> {
        let sensor_types = sqlx::query!(
            r#"
            SELECT id, type as "type_"
            FROM Sensor_type
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        let sensor_types = sensor_types
            .into_iter()
            .map(|st| SensorType {
                id: st.id,
                type_: st.type_,
            })
            .collect();

        Ok(sensor_types)
    }

    pub async fn update_sensor_type(
        pool: &PgPool,
        id: i32,
        sensor_type_dto: UpdateSensorTypeDto,
    ) -> Result<SensorType, AppError> {
        // Prepare the update
        let sensor_type_update = SensorTypeUpdate {
            type_: sensor_type_dto.type_,
        };

        // Update the sensor type
        let sensor_type = sqlx::query!(
            r#"
            UPDATE Sensor_type
            SET 
                type = COALESCE($1, type)
            WHERE id = $2
            RETURNING id, type as "type_"
            "#,
            sensor_type_update.type_,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(SensorType {
            id: sensor_type.id,
            type_: sensor_type.type_,
        })
    }

    pub async fn delete_sensor_type(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Sensor_type
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

