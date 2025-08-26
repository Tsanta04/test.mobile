use crate::dto::sensor_pack::{CreateSensorPackDto, UpdateSensorPackDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::sensor_pack::{SensorPack, NewSensorPack, SensorPackUpdate};
use sqlx::PgPool;

pub struct SensorPackService;

impl SensorPackService {
    pub async fn create_sensor_pack(
        pool: &PgPool,
        sensor_pack_dto: CreateSensorPackDto,
    ) -> Result<SensorPack, AppError> {
        // Create a new sensor pack
        let new_sensor_pack = NewSensorPack {
            id: sensor_pack_dto.id,
            description: sensor_pack_dto.description,
        };

        // Insert the sensor pack into the database
        let sensor_pack = sqlx::query_as!(
            SensorPack,
            r#"
            INSERT INTO Sensor_pack (id, description)
            VALUES ($1, $2)
            RETURNING id, description
            "#,
            new_sensor_pack.id,
            new_sensor_pack.description
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensor_pack)
    }

    pub async fn get_sensor_pack_by_id(pool: &PgPool, id: &str) -> Result<SensorPack, AppError> {
        let sensor_pack = sqlx::query_as!(
            SensorPack,
            r#"
            SELECT id, description
            FROM Sensor_pack
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensor_pack)
    }

    pub async fn get_all_sensor_packs(pool: &PgPool) -> Result<Vec<SensorPack>, AppError> {
        let sensor_packs = sqlx::query_as!(
            SensorPack,
            r#"
            SELECT id, description
            FROM Sensor_pack
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensor_packs)
    }

    pub async fn update_sensor_pack(
        pool: &PgPool,
        id: &str,
        sensor_pack_dto: UpdateSensorPackDto,
    ) -> Result<SensorPack, AppError> {
        // Prepare the update
        let sensor_pack_update = SensorPackUpdate {
            description: sensor_pack_dto.description,
        };

        // Update the sensor pack
        let sensor_pack = sqlx::query_as!(
            SensorPack,
            r#"
            UPDATE Sensor_pack
            SET 
                description = COALESCE($1, description)
            WHERE id = $2
            RETURNING id, description
            "#,
            sensor_pack_update.description,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensor_pack)
    }

    pub async fn delete_sensor_pack(pool: &PgPool, id: &str) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Sensor_pack
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

