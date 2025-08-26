use crate::models::location::Location;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateLocationDto {
    pub longitude: f64,
    pub latitude: f64,
    
    #[validate(length(min = 1, max = 200, message = "City must be between 1 and 200 characters"))]
    pub city: String,
    
    #[validate(length(min = 1, max = 200, message = "Country must be between 1 and 200 characters"))]
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateLocationDto {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    
    #[validate(length(min = 1, max = 200, message = "City must be between 1 and 200 characters"))]
    pub city: Option<String>,
    
    #[validate(length(min = 1, max = 200, message = "Country must be between 1 and 200 characters"))]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponseDto {
    pub id: i32,
    pub longitude: f64,
    pub latitude: f64,
    pub city: String,
    pub country: String,
}

impl From<Location> for LocationResponseDto {
    fn from(location: Location) -> Self {
        Self {
            id: location.id,
            longitude: location.longitude,
            latitude: location.latitude,
            city: location.city,
            country: location.country,
        }
    }
}

