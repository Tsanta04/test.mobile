use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationDto {
    pub id: i32,
    pub longitude: f64,
    pub latitude: f64,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocationDto {
    pub longitude: f64,
    pub latitude: f64,
    pub city: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLocationDto {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub city: Option<String>,
    pub country: Option<String>,
}

