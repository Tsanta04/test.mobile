use crate::dto::pack::{CreatePackDto, UpdatePackDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::pack::{Pack, NewPack, PackUpdate};
use sqlx::PgPool;

pub struct PackService;

impl PackService {
    pub async fn create_pack(
        pool: &PgPool,
        pack_dto: CreatePackDto,
    ) -> Result<Pack, AppError> {
        // Create a new pack
        let new_pack = NewPack {
            pack_id: pack_dto.pack_id,
            sensor_id: pack_dto.sensor_id,
        };

        // Insert the pack into the database
        let pack = sqlx::query_as!(
            Pack,
            r#"
            INSERT INTO Pack (pack_id, sensor_id)
            VALUES ($1, $2)
            RETURNING id, pack_id, sensor_id
            "#,
            new_pack.pack_id,
            new_pack.sensor_id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(pack)
    }

    pub async fn get_pack_by_id(pool: &PgPool, id: i32) -> Result<Pack, AppError> {
        let pack = sqlx::query_as!(
            Pack,
            r#"
            SELECT id, pack_id, sensor_id
            FROM Pack
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(pack)
    }

    pub async fn get_packs_by_pack_id(pool: &PgPool, pack_id: &str) -> Result<Vec<Pack>, AppError> {
        let packs = sqlx::query_as!(
            Pack,
            r#"
            SELECT id, pack_id, sensor_id
            FROM Pack
            WHERE pack_id = $1
            ORDER BY id
            "#,
            pack_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(packs)
    }

    pub async fn get_packs_by_sensor_id(pool: &PgPool, sensor_id: i32) -> Result<Vec<Pack>, AppError> {
        let packs = sqlx::query_as!(
            Pack,
            r#"
            SELECT id, pack_id, sensor_id
            FROM Pack
            WHERE sensor_id = $1
            ORDER BY id
            "#,
            sensor_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(packs)
    }

    pub async fn get_all_packs(pool: &PgPool) -> Result<Vec<Pack>, AppError> {
        let packs = sqlx::query_as!(
            Pack,
            r#"
            SELECT id, pack_id, sensor_id
            FROM Pack
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(packs)
    }

    pub async fn update_pack(
        pool: &PgPool,
        id: i32,
        pack_dto: UpdatePackDto,
    ) -> Result<Pack, AppError> {
        // Prepare the update
        let pack_update = PackUpdate {
            pack_id: pack_dto.pack_id,
            sensor_id: pack_dto.sensor_id,
        };

        // Update the pack
        let pack = sqlx::query_as!(
            Pack,
            r#"
            UPDATE Pack
            SET 
                pack_id = COALESCE($1, pack_id),
                sensor_id = COALESCE($2, sensor_id)
            WHERE id = $3
            RETURNING id, pack_id, sensor_id
            "#,
            pack_update.pack_id,
            pack_update.sensor_id,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(pack)
    }

    pub async fn delete_pack(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Pack
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

