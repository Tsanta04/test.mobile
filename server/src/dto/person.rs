use crate::models::person::Person;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePersonDto {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: String,
    
    #[validate(length(max = 200, message = "First name must be less than 200 characters"))]
    pub firstname: Option<String>,
    
    pub date_birth: Option<DateTime<Utc>>,
    
    #[validate(length(max = 200, message = "Birth location must be less than 200 characters"))]
    pub location_birth: Option<String>,
    
    #[validate(length(max = 10, message = "Number must be less than 10 characters"))]
    pub number: Option<String>,
    
    #[validate(length(max = 15, message = "CIN must be less than 15 characters"))]
    pub cin: Option<String>,
    
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePersonDto {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: Option<String>,
    
    #[validate(length(max = 200, message = "First name must be less than 200 characters"))]
    pub firstname: Option<String>,
    
    pub date_birth: Option<DateTime<Utc>>,
    
    #[validate(length(max = 200, message = "Birth location must be less than 200 characters"))]
    pub location_birth: Option<String>,
    
    #[validate(length(max = 10, message = "Number must be less than 10 characters"))]
    pub number: Option<String>,
    
    #[validate(length(max = 15, message = "CIN must be less than 15 characters"))]
    pub cin: Option<String>,
    
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResponseDto {
    pub id: i32,
    pub name: String,
    pub firstname: Option<String>,
    pub date_birth: Option<DateTime<Utc>>,
    pub location_birth: Option<String>,
    pub number: Option<String>,
    pub cin: Option<String>,
    pub user_id: Option<i32>,
}

impl From<Person> for PersonResponseDto {
    fn from(person: Person) -> Self {
        Self {
            id: person.id,
            name: person.name,
            firstname: person.firstname,
            date_birth: person.date_birth,
            location_birth: person.location_birth,
            number: person.number,
            cin: person.cin,
            user_id: person.user_id,
        }
    }
}

