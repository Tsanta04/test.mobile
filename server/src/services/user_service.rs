use crate::error::AppError;
use crate::models::user::{NewUser, User, UserUpdate};
use crate::schema::users;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct UserService;

impl UserService {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<User>, AppError> {
        users::table
            .select(User::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<User, AppError> {
        users::table
            .find(id)
            .select(User::as_select())
            .first(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_username(username: &str, conn: &mut PgConnection) -> Result<User, AppError> {
        users::table
            .filter(users::username.eq(username))
            .select(User::as_select())
            .first(conn)
            .map_err(AppError::from)
    }

    pub fn create(new_user: NewUser, conn: &mut PgConnection) -> Result<User, AppError> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn update(id: i32, user_update: UserUpdate, conn: &mut PgConnection) -> Result<User, AppError> {
        diesel::update(users::table.find(id))
            .set(&user_update)
            .returning(User::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, AppError> {
        diesel::delete(users::table.find(id))
            .execute(conn)
            .map_err(AppError::from)
    }
}

