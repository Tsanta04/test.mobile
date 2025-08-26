use crate::error::AppError;
use crate::models::ground::{Ground, GroundUpdate, NewGround};
use crate::schema::ground;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct GroundService;

impl GroundService {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Ground>, AppError> {
        ground::table
            .select(Ground::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<Ground, AppError> {
        ground::table
            .find(id)
            .select(Ground::as_select())
            .first(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_user(user_id: i32, conn: &mut PgConnection) -> Result<Vec<Ground>, AppError> {
        ground::table
            .filter(ground::user_id.eq(user_id))
            .select(Ground::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_culture_type(culture_type_id: i32, conn: &mut PgConnection) -> Result<Vec<Ground>, AppError> {
        ground::table
            .filter(ground::culture_type.eq(culture_type_id))
            .select(Ground::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_location(location_id: i32, conn: &mut PgConnection) -> Result<Vec<Ground>, AppError> {
        ground::table
            .filter(ground::location.eq(location_id))
            .select(Ground::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_pack(pack_id: &str, conn: &mut PgConnection) -> Result<Vec<Ground>, AppError> {
        ground::table
            .filter(ground::pack.eq(pack_id))
            .select(Ground::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn create(new_ground: NewGround, conn: &mut PgConnection) -> Result<Ground, AppError> {
        diesel::insert_into(ground::table)
            .values(&new_ground)
            .returning(Ground::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn update(id: i32, ground_update: GroundUpdate, conn: &mut PgConnection) -> Result<Ground, AppError> {
        diesel::update(ground::table.find(id))
            .set(&ground_update)
            .returning(Ground::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, AppError> {
        diesel::delete(ground::table.find(id))
            .execute(conn)
            .map_err(AppError::from)
    }
}

