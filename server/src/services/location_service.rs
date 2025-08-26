use crate::dto::{CreateLocationDto, LocationDto, UpdateLocationDto};
use crate::errors::AppError;
use crate::models::{Location, NewLocation, UpdateLocation};
use crate::schema::location;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct LocationService {
    base: Service,
}

impl LocationService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_location(&self, dto: CreateLocationDto) -> Result<LocationDto, AppError> {
        use crate::schema::location::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_location = NewLocation {
            longitude: dto.longitude,
            latitude: dto.latitude,
            city: dto.city,
            country: dto.country,
        };

        let location_result = diesel::insert_into(location)
            .values(&new_location)
            .get_result::<Location>(conn)?;

        Ok(LocationDto {
            id: location_result.id,
            longitude: location_result.longitude,
            latitude: location_result.latitude,
            city: location_result.city,
            country: location_result.country,
        })
    }

    pub async fn get_location_by_id(&self, location_id: i32) -> Result<LocationDto, AppError> {
        use crate::schema::location::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let location_result = location
            .find(location_id)
            .first::<Location>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Location not found".to_string()))?;

        Ok(LocationDto {
            id: location_result.id,
            longitude: location_result.longitude,
            latitude: location_result.latitude,
            city: location_result.city,
            country: location_result.country,
        })
    }

    pub async fn get_all_locations(&self) -> Result<Vec<LocationDto>, AppError> {
        use crate::schema::location::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = location.load::<Location>(conn)?;

        let location_dtos = results
            .into_iter()
            .map(|loc| LocationDto {
                id: loc.id,
                longitude: loc.longitude,
                latitude: loc.latitude,
                city: loc.city,
                country: loc.country,
            })
            .collect();

        Ok(location_dtos)
    }

    pub async fn update_location(
        &self,
        location_id: i32,
        dto: UpdateLocationDto,
    ) -> Result<LocationDto, AppError> {
        use crate::schema::location::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if location exists
        let location_exists = location
            .find(location_id)
            .first::<Location>(conn)
            .optional()?
            .is_some();

        if !location_exists {
            return Err(AppError::NotFound("Location not found".to_string()));
        }

        let update_location = UpdateLocation {
            longitude: dto.longitude,
            latitude: dto.latitude,
            city: dto.city,
            country: dto.country,
        };

        let updated_location = diesel::update(location.find(location_id))
            .set(&update_location)
            .get_result::<Location>(conn)?;

        Ok(LocationDto {
            id: updated_location.id,
            longitude: updated_location.longitude,
            latitude: updated_location.latitude,
            city: updated_location.city,
            country: updated_location.country,
        })
    }

    pub async fn delete_location(&self, location_id: i32) -> Result<(), AppError> {
        use crate::schema::location::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if location exists
        let location_exists = location
            .find(location_id)
            .first::<Location>(conn)
            .optional()?
            .is_some();

        if !location_exists {
            return Err(AppError::NotFound("Location not found".to_string()));
        }

        diesel::delete(location.find(location_id)).execute(conn)?;

        Ok(())
    }
}

impl BaseService for LocationService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

