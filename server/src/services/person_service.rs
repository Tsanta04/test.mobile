use crate::dto::{CreatePersonDto, PersonDto, UpdatePersonDto};
use crate::errors::AppError;
use crate::models::{NewPerson, Person, UpdatePerson};
use crate::schema::person;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct PersonService {
    base: Service,
}

impl PersonService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_person(&self, dto: CreatePersonDto) -> Result<PersonDto, AppError> {
        use crate::schema::person::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_person = NewPerson {
            name: dto.name,
            firstname: dto.firstname,
            date_birth: dto.date_birth,
            location_birth: dto.location_birth,
            number: dto.number,
            cin: dto.cin,
            user_id: dto.user_id,
        };

        let person_result = diesel::insert_into(person)
            .values(&new_person)
            .get_result::<Person>(conn)?;

        Ok(PersonDto {
            id: person_result.id,
            name: person_result.name,
            firstname: person_result.firstname,
            date_birth: person_result.date_birth,
            location_birth: person_result.location_birth,
            number: person_result.number,
            cin: person_result.cin,
            user_id: person_result.user_id,
        })
    }

    pub async fn get_person_by_id(&self, person_id: i32) -> Result<PersonDto, AppError> {
        use crate::schema::person::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let person_result = person
            .find(person_id)
            .first::<Person>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Person not found".to_string()))?;

        Ok(PersonDto {
            id: person_result.id,
            name: person_result.name,
            firstname: person_result.firstname,
            date_birth: person_result.date_birth,
            location_birth: person_result.location_birth,
            number: person_result.number,
            cin: person_result.cin,
            user_id: person_result.user_id,
        })
    }

    pub async fn get_all_persons(&self) -> Result<Vec<PersonDto>, AppError> {
        use crate::schema::person::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = person.load::<Person>(conn)?;

        let person_dtos = results
            .into_iter()
            .map(|p| PersonDto {
                id: p.id,
                name: p.name,
                firstname: p.firstname,
                date_birth: p.date_birth,
                location_birth: p.location_birth,
                number: p.number,
                cin: p.cin,
                user_id: p.user_id,
            })
            .collect();

        Ok(person_dtos)
    }

    pub async fn update_person(
        &self,
        person_id: i32,
        dto: UpdatePersonDto,
    ) -> Result<PersonDto, AppError> {
        use crate::schema::person::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if person exists
        let person_exists = person
            .find(person_id)
            .first::<Person>(conn)
            .optional()?
            .is_some();

        if !person_exists {
            return Err(AppError::NotFound("Person not found".to_string()));
        }

        let update_person = UpdatePerson {
            name: dto.name,
            firstname: dto.firstname,
            date_birth: dto.date_birth,
            location_birth: dto.location_birth,
            number: dto.number,
            cin: dto.cin,
            user_id: dto.user_id,
        };

        let updated_person = diesel::update(person.find(person_id))
            .set(&update_person)
            .get_result::<Person>(conn)?;

        Ok(PersonDto {
            id: updated_person.id,
            name: updated_person.name,
            firstname: updated_person.firstname,
            date_birth: updated_person.date_birth,
            location_birth: updated_person.location_birth,
            number: updated_person.number,
            cin: updated_person.cin,
            user_id: updated_person.user_id,
        })
    }

    pub async fn delete_person(&self, person_id: i32) -> Result<(), AppError> {
        use crate::schema::person::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if person exists
        let person_exists = person
            .find(person_id)
            .first::<Person>(conn)
            .optional()?
            .is_some();

        if !person_exists {
            return Err(AppError::NotFound("Person not found".to_string()));
        }

        diesel::delete(person.find(person_id)).execute(conn)?;

        Ok(())
    }

    pub async fn get_person_by_user_id(&self, user_id_val: i32) -> Result<PersonDto, AppError> {
        use crate::schema::person::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let person_result = person
            .filter(user_id.eq(user_id_val))
            .first::<Person>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Person not found for this user".to_string()))?;

        Ok(PersonDto {
            id: person_result.id,
            name: person_result.name,
            firstname: person_result.firstname,
            date_birth: person_result.date_birth,
            location_birth: person_result.location_birth,
            number: person_result.number,
            cin: person_result.cin,
            user_id: person_result.user_id,
        })
    }
}

impl BaseService for PersonService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

