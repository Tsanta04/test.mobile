use crate::dto::culture_type::{CreateCultureTypeDto, UpdateCultureTypeDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::culture_type::{CultureType, NewCultureType, CultureTypeUpdate};
use sqlx::PgPool;

pub struct CultureTypeService;

impl CultureTypeService {
    pub async fn create_culture_type(
        pool: &PgPool,
        culture_type_dto: CreateCultureTypeDto,
    ) -> Result<CultureType, AppError> {
        // Create a new culture type
        let new_culture_type = NewCultureType {
            type_: culture_type_dto.type_,
        };

        // Insert the culture type into the database
        let culture_type = sqlx::query!(
            r#"
            INSERT INTO Culture_type (type)
            VALUES ($1)
            RETURNING id, type as "type_"
            "#,
            new_culture_type.type_
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(CultureType {
            id: culture_type.id,
            type_: culture_type.type_,
        })
    }

    pub async fn get_culture_type_by_id(pool: &PgPool, id: i32) -> Result<CultureType, AppError> {
        let culture_type = sqlx::query!(
            r#"
            SELECT id, type as "type_"
            FROM Culture_type
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(CultureType {
            id: culture_type.id,
            type_: culture_type.type_,
        })
    }

    pub async fn get_all_culture_types(pool: &PgPool) -> Result<Vec<CultureType>, AppError> {
        let culture_types = sqlx::query!(
            r#"
            SELECT id, type as "type_"
            FROM Culture_type
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        let culture_types = culture_types
            .into_iter()
            .map(|ct| CultureType {
                id: ct.id,
                type_: ct.type_,
            })
            .collect();

        Ok(culture_types)
    }

    pub async fn update_culture_type(
        pool: &PgPool,
        id: i32,
        culture_type_dto: UpdateCultureTypeDto,
    ) -> Result<CultureType, AppError> {
        // Prepare the update
        let culture_type_update = CultureTypeUpdate {
            type_: culture_type_dto.type_,
        };

        // Update the culture type
        let culture_type = sqlx::query!(
            r#"
            UPDATE Culture_type
            SET 
                type = COALESCE($1, type)
            WHERE id = $2
            RETURNING id, type as "type_"
            "#,
            culture_type_update.type_,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(CultureType {
            id: culture_type.id,
            type_: culture_type.type_,
        })
    }

    pub async fn delete_culture_type(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Culture_type
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

