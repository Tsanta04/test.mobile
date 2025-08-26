use crate::dto::sensor::{CreateSensorDto, UpdateSensorDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::sensor::{Sensor, NewSensor, SensorUpdate};
use sqlx::PgPool;

pub struct SensorService;

impl SensorService {
    pub async fn create_sensor(
        pool: &PgPool,
        sensor_dto: CreateSensorDto,
    ) -> Result<Sensor, AppError> {
        // Create a new sensor
        let new_sensor = NewSensor {
            description: sensor_dto.description,
            issue_date: sensor_dto.issue_date,
            sensor_type: sensor_dto.sensor_type,
        };

        // Insert the sensor into the database
        let sensor = sqlx::query_as!(
            Sensor,
            r#"
            INSERT INTO Sensor (description, issue_date, sensor_type)
            VALUES ($1, $2, $3)
            RETURNING id, description, issue_date, sensor_type
            "#,
            new_sensor.description,
            new_sensor.issue_date,
            new_sensor.sensor_type
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensor)
    }

    pub async fn get_sensor_by_id(pool: &PgPool, id: i32) -> Result<Sensor, AppError> {
        let sensor = sqlx::query_as!(
            Sensor,
            r#"
            SELECT id, description, issue_date, sensor_type
            FROM Sensor
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensor)
    }

    pub async fn get_sensors_by_type(pool: &PgPool, sensor_type_id: i32) -> Result<Vec<Sensor>, AppError> {
        let sensors = sqlx::query_as!(
            Sensor,
            r#"
            SELECT id, description, issue_date, sensor_type
            FROM Sensor
            WHERE sensor_type = $1
            ORDER BY id
            "#,
            sensor_type_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensors)
    }

    pub async fn get_all_sensors(pool: &PgPool) -> Result<Vec<Sensor>, AppError> {
        let sensors = sqlx::query_as!(
            Sensor,
            r#"
            SELECT id, description, issue_date, sensor_type
            FROM Sensor
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensors)
    }

    pub async fn update_sensor(
        pool: &PgPool,
        id: i32,
        sensor_dto: UpdateSensorDto,
    ) -> Result<Sensor, AppError> {
        // Prepare the update
        let sensor_update = SensorUpdate {
            description: sensor_dto.description,
            issue_date: sensor_dto.issue_date,
            sensor_type: sensor_dto.sensor_type,
        };

        // Update the sensor
        let sensor = sqlx::query_as!(
            Sensor,
            r#"
            UPDATE Sensor
            SET 
                description = COALESCE($1, description),
                issue_date = COALESCE($2, issue_date),
                sensor_type = COALESCE($3, sensor_type)
            WHERE id = $4
            RETURNING id, description, issue_date, sensor_type
            "#,
            sensor_update.description,
            sensor_update.issue_date,
            sensor_update.sensor_type,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(sensor)
    }

    pub async fn delete_sensor(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Sensor
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

