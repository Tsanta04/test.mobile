use crate::dto::{CreateDiscussionDto, DiscussionDto, UpdateDiscussionDto};
use crate::errors::AppError;
use crate::models::{Discussion, NewDiscussion, UpdateDiscussion};
use crate::schema::discussion;
use crate::services::base_service::{BaseService, DbPool, Service};
use diesel::prelude::*;
use std::sync::Arc;

pub struct DiscussionService {
    base: Service,
}

impl DiscussionService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self {
            base: Service::new(pool),
        }
    }

    pub async fn create_discussion(
        &self,
        dto: CreateDiscussionDto,
    ) -> Result<DiscussionDto, AppError> {
        use crate::schema::discussion::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let new_discussion = NewDiscussion {
            name: dto.name,
            initialised_at: dto.initialised_at,
        };

        let discussion_result = diesel::insert_into(discussion)
            .values(&new_discussion)
            .get_result::<Discussion>(conn)?;

        Ok(DiscussionDto {
            id: discussion_result.id,
            name: discussion_result.name,
            initialised_at: discussion_result.initialised_at,
        })
    }

    pub async fn get_discussion_by_id(&self, discussion_id: i32) -> Result<DiscussionDto, AppError> {
        use crate::schema::discussion::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let discussion_result = discussion
            .find(discussion_id)
            .first::<Discussion>(conn)
            .optional()?
            .ok_or_else(|| AppError::NotFound("Discussion not found".to_string()))?;

        Ok(DiscussionDto {
            id: discussion_result.id,
            name: discussion_result.name,
            initialised_at: discussion_result.initialised_at,
        })
    }

    pub async fn get_all_discussions(&self) -> Result<Vec<DiscussionDto>, AppError> {
        use crate::schema::discussion::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        let results = discussion.load::<Discussion>(conn)?;

        let discussion_dtos = results
            .into_iter()
            .map(|d| DiscussionDto {
                id: d.id,
                name: d.name,
                initialised_at: d.initialised_at,
            })
            .collect();

        Ok(discussion_dtos)
    }

    pub async fn update_discussion(
        &self,
        discussion_id: i32,
        dto: UpdateDiscussionDto,
    ) -> Result<DiscussionDto, AppError> {
        use crate::schema::discussion::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if discussion exists
        let discussion_exists = discussion
            .find(discussion_id)
            .first::<Discussion>(conn)
            .optional()?
            .is_some();

        if !discussion_exists {
            return Err(AppError::NotFound("Discussion not found".to_string()));
        }

        let update_discussion = UpdateDiscussion {
            name: dto.name,
            initialised_at: dto.initialised_at,
        };

        let updated_discussion = diesel::update(discussion.find(discussion_id))
            .set(&update_discussion)
            .get_result::<Discussion>(conn)?;

        Ok(DiscussionDto {
            id: updated_discussion.id,
            name: updated_discussion.name,
            initialised_at: updated_discussion.initialised_at,
        })
    }

    pub async fn delete_discussion(&self, discussion_id: i32) -> Result<(), AppError> {
        use crate::schema::discussion::dsl::*;

        let conn = &mut self.base.get_pool().get()?;

        // Check if discussion exists
        let discussion_exists = discussion
            .find(discussion_id)
            .first::<Discussion>(conn)
            .optional()?
            .is_some();

        if !discussion_exists {
            return Err(AppError::NotFound("Discussion not found".to_string()));
        }

        diesel::delete(discussion.find(discussion_id)).execute(conn)?;

        Ok(())
    }
}

impl BaseService for DiscussionService {
    fn get_pool(&self) -> Arc<DbPool> {
        self.base.get_pool()
    }
}

