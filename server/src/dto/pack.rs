use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackDto {
    pub id: i32,
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePackDto {
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePackDto {
    pub pack_id: Option<String>,
    pub sensor_id: Option<i32>,
}

