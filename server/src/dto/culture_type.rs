use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CultureTypeDto {
    pub id: i32,
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCultureTypeDto {
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCultureTypeDto {
    pub type_: Option<String>,
}

