use crate::dto::person::{CreatePersonDto, UpdatePersonDto};
use crate::error::{map_sqlx_error, AppError};
use crate::models::person::{NewPerson, Person, PersonUpdate};
use sqlx::PgPool;

pub struct PersonService;

impl PersonService {
    pub async fn create_person(pool: &PgPool, person_dto: CreatePersonDto) -> Result<Person, AppError> {
        // Create a new person
        let new_person = NewPerson {
            name: person_dto.name,
            firstname: person_dto.firstname,
            date_birth: person_dto.date_birth,
            location_birth: person_dto.location_birth,
            number: person_dto.number,
            cin: person_dto.cin,
            user_id: person_dto.user_id,
        };

        // Insert the person into the database
        let person = sqlx::query_as!(
            Person,
            r#"
            INSERT INTO Person (name, firstname, date_birth, location_birth, number, cin, user_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, firstname, date_birth, location_birth, number, cin, user_id
            "#,
            new_person.name,
            new_person.firstname,
            new_person.date_birth,
            new_person.location_birth,
            new_person.number,
            new_person.cin,
            new_person.user_id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(person)
    }

    pub async fn get_person_by_id(pool: &PgPool, id: i32) -> Result<Person, AppError> {
        let person = sqlx::query_as!(
            Person,
            r#"
            SELECT id, name, firstname, date_birth, location_birth, number, cin, user_id
            FROM Person
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(person)
    }

    pub async fn get_person_by_user_id(pool: &PgPool, user_id: i32) -> Result<Person, AppError> {
        let person = sqlx::query_as!(
            Person,
            r#"
            SELECT id, name, firstname, date_birth, location_birth, number, cin, user_id
            FROM Person
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(person)
    }

    pub async fn get_all_persons(pool: &PgPool) -> Result<Vec<Person>, AppError> {
        let persons = sqlx::query_as!(
            Person,
            r#"
            SELECT id, name, firstname, date_birth, location_birth, number, cin, user_id
            FROM Person
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(persons)
    }

    pub async fn update_person(
        pool: &PgPool,
        id: i32,
        person_dto: UpdatePersonDto,
    ) -> Result<Person, AppError> {
        // Prepare the update
        let person_update = PersonUpdate {
            name: person_dto.name,
            firstname: person_dto.firstname,
            date_birth: person_dto.date_birth,
            location_birth: person_dto.location_birth,
            number: person_dto.number,
            cin: person_dto.cin,
            user_id: person_dto.user_id,
        };

        // Update the person
        let person = sqlx::query_as!(
            Person,
            r#"
            UPDATE Person
            SET 
                name = COALESCE($1, name),
                firstname = COALESCE($2, firstname),
                date_birth = COALESCE($3, date_birth),
                location_birth = COALESCE($4, location_birth),
                number = COALESCE($5, number),
                cin = COALESCE($6, cin),
                user_id = COALESCE($7, user_id)
            WHERE id = $8
            RETURNING id, name, firstname, date_birth, location_birth, number, cin, user_id
            "#,
            person_update.name,
            person_update.firstname,
            person_update.date_birth,
            person_update.location_birth,
            person_update.number,
            person_update.cin,
            person_update.user_id,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(person)
    }

    pub async fn delete_person(pool: &PgPool, id: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM Person
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

