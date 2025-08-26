use crate::dto::planning::{CreatePlanningDto, UpdatePlanningDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::planning::{Planning, NewPlanning, PlanningUpdate};
use sqlx::PgPool;

pub struct PlanningService;

impl PlanningService {
    pub async fn create_planning(
        pool: &PgPool,
        planning_dto: CreatePlanningDto,
    ) -> Result<Planning, AppError> {
        // Create a new planning
        let new_planning = NewPlanning {
            title: planning_dto.title,
            description: planning_dto.description,
            start_date: planning_dto.start_date,
            end_date: planning_dto.end_date,
            ground: planning_dto.ground,
        };

        // Insert the planning into the database
        let planning = sqlx::query_as!(
            Planning,
            r#"
            INSERT INTO Planning (title, description, start_date, end_date, ground)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, date, title, description, start_date, end_date, ground
            "#,
            new_planning.title,
            new_planning.description,
            new_planning.start_date,
            new_planning.end_date,
            new_planning.ground
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(planning)
    }

    pub async fn get_planning_by_id(pool: &PgPool, id: i32) -> Result<Planning, AppError> {
        let planning = sqlx::query_as!(
            Planning,
            r#"
            SELECT id, date, title, description, start_date, end_date, ground
            FROM Planning
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(planning)
    }

    pub async fn get_plannings_by_ground(pool: &PgPool, ground_id: i32) -> Result<Vec<Planning>, AppError> {
        let plannings = sqlx::query_as!(
            Planning,
            r#"
            SELECT id, date, title, description, start_date, end_date, ground
            FROM Planning
            WHERE ground = $1
            ORDER BY start_date
            "#,
            ground_id
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(plannings)
    }

    pub async fn get_plannings_by_date_range(
        pool: &PgPool,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Planning>, AppError> {
        let plannings = sqlx::query_as!(
            Planning,
            r#"
            SELECT id, date, title, description, start_date, end_date, ground
            FROM Planning
            WHERE 
                (start_date BETWEEN $1 AND $2) OR
                (end_date BETWEEN $1 AND $2) OR
                (start_date <= $1 AND end_date >= $2)
            ORDER BY start_date
            "#,
            start_date,
            end_date
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(plannings)
    }

    pub async fn get_all_plannings(pool: &PgPool) -> Result<Vec<Planning>, AppError> {
        let plannings = sqlx::query_as!(
            Planning,
            r#"
            SELECT id, date, title, description, start_date, end_date, ground
            FROM Planning
            ORDER BY start_date
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(plannings)
    }

    pub async fn update_planning(
        pool: &PgPool,
        id: i32,
        planning_dto: UpdatePlanningDto,
    ) -> Result<Planning, AppError> {
        // Prepare the update
        let planning_update = PlanningUpdate {
            title: planning_dto.title,
            description: planning_dto.description,
            start_date: planning_dto.start_date,
            end_date: planning_dto.end_date,
            ground: planning_dto.ground,
        };

        // Update the planning
        let planning = sqlx::query_as!(
            Planning,
            r#"
            UPDATE Planning
            SET 
                title = COALESCE($1, title),
                description = COALESCE($2, description),
                start_date = COALESCE($3, start_date),
                end_date = COALESCE($4, end_date),
                ground = COALESCE($5, ground)
            WHERE id = $6
            RETURNING id, date, title, description, start_date, end_date, ground
            "#,
            planning_update.title,
            planning_update.description,
            planning_update.start_date,
            planning_update.end_date,
            planning_update.ground,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(planning)
    }

    pub async fn delete_planning(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Planning
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

