use crate::dto::location::{CreateLocationDto, UpdateLocationDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::location::{Location, NewLocation, LocationUpdate};
use sqlx::PgPool;

pub struct LocationService;

impl LocationService {
    pub async fn create_location(
        pool: &PgPool,
        location_dto: CreateLocationDto,
    ) -> Result<Location, AppError> {
        // Create a new location
        let new_location = NewLocation {
            longitude: location_dto.longitude,
            latitude: location_dto.latitude,
            city: location_dto.city,
            country: location_dto.country,
        };

        // Insert the location into the database
        let location = sqlx::query_as!(
            Location,
            r#"
            INSERT INTO Location (longitude, latitude, city, country)
            VALUES ($1, $2, $3, $4)
            RETURNING id, longitude, latitude, city, country
            "#,
            new_location.longitude,
            new_location.latitude,
            new_location.city,
            new_location.country
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(location)
    }

    pub async fn get_location_by_id(pool: &PgPool, id: i32) -> Result<Location, AppError> {
        let location = sqlx::query_as!(
            Location,
            r#"
            SELECT id, longitude, latitude, city, country
            FROM Location
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(location)
    }

    pub async fn get_all_locations(pool: &PgPool) -> Result<Vec<Location>, AppError> {
        let locations = sqlx::query_as!(
            Location,
            r#"
            SELECT id, longitude, latitude, city, country
            FROM Location
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(locations)
    }

    pub async fn update_location(
        pool: &PgPool,
        id: i32,
        location_dto: UpdateLocationDto,
    ) -> Result<Location, AppError> {
        // Prepare the update
        let location_update = LocationUpdate {
            longitude: location_dto.longitude,
            latitude: location_dto.latitude,
            city: location_dto.city,
            country: location_dto.country,
        };

        // Update the location
        let location = sqlx::query_as!(
            Location,
            r#"
            UPDATE Location
            SET 
                longitude = COALESCE($1, longitude),
                latitude = COALESCE($2, latitude),
                city = COALESCE($3, city),
                country = COALESCE($4, country)
            WHERE id = $5
            RETURNING id, longitude, latitude, city, country
            "#,
            location_update.longitude,
            location_update.latitude,
            location_update.city,
            location_update.country,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(location)
    }

    pub async fn delete_location(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Location
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

