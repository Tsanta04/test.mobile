use crate::errors::AppError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub trait BaseService {
    fn get_pool(&self) -> Arc<DbPool>;
}

pub struct Service {
    pool: Arc<DbPool>,
}

impl Service {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

impl BaseService for Service {
    fn get_pool(&self) -> Arc<DbPool> {
        self.pool.clone()
    }
}

