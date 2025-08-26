use crate::models::culture_type::CultureType;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCultureTypeDto {
    #[validate(length(min = 1, max = 200, message = "Type must be between 1 and 200 characters"))]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCultureTypeDto {
    #[validate(length(min = 1, max = 200, message = "Type must be between 1 and 200 characters"))]
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CultureTypeResponseDto {
    pub id: i32,
    pub type_: String,
}

impl From<CultureType> for CultureTypeResponseDto {
    fn from(culture_type: CultureType) -> Self {
        Self {
            id: culture_type.id,
            type_: culture_type.type_,
        }
    }
}

