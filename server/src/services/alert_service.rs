use crate::error::AppError;
use crate::models::alert::{Alert, AlertType, AlertUpdate, NewAlert};
use crate::schema::alert;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct AlertService;

impl AlertService {
    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Alert>, AppError> {
        alert::table
            .select(Alert::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<Alert, AppError> {
        alert::table
            .find(id)
            .select(Alert::as_select())
            .first(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_state(state_id: i32, conn: &mut PgConnection) -> Result<Vec<Alert>, AppError> {
        alert::table
            .filter(alert::state_id.eq(state_id))
            .select(Alert::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_by_type(alert_type: &str, conn: &mut PgConnection) -> Result<Vec<Alert>, AppError> {
        let alert_type = match alert_type {
            "Health" => AlertType::Health,
            "Production" => AlertType::Production,
            "Rentability" => AlertType::Rentability,
            "Humidity" => AlertType::Humidity,
            "Fertility" => AlertType::Fertility,
            _ => AlertType::Other,
        };

        alert::table
            .filter(alert::type_.eq(alert_type))
            .select(Alert::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn find_unseen(conn: &mut PgConnection) -> Result<Vec<Alert>, AppError> {
        alert::table
            .filter(alert::isseen.eq(false))
            .select(Alert::as_select())
            .load(conn)
            .map_err(AppError::from)
    }

    pub fn create(new_alert: NewAlert, conn: &mut PgConnection) -> Result<Alert, AppError> {
        diesel::insert_into(alert::table)
            .values(&new_alert)
            .returning(Alert::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn update(id: i32, alert_update: AlertUpdate, conn: &mut PgConnection) -> Result<Alert, AppError> {
        diesel::update(alert::table.find(id))
            .set(&alert_update)
            .returning(Alert::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn mark_as_seen(id: i32, conn: &mut PgConnection) -> Result<Alert, AppError> {
        diesel::update(alert::table.find(id))
            .set(alert::isseen.eq(true))
            .returning(Alert::as_returning())
            .get_result(conn)
            .map_err(AppError::from)
    }

    pub fn delete(id: i32, conn: &mut PgConnection) -> Result<usize, AppError> {
        diesel::delete(alert::table.find(id))
            .execute(conn)
            .map_err(AppError::from)
    }
}

