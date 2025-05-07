use crate::{
    application::{
        dto::response::user::GetUserResponse,
        mappers::{application::ApplicationMapper, user::get_user::GetUserMapper},
        repositories::users::get_user::GetUserRepository,
    },
    domain::errors::repositories_errors::{RepositoryError, RepositoryResult},
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct GetUserServiceImpl {
    pub repository: Arc<dyn GetUserRepository>,
}
impl GetUserServiceImpl {
    pub fn new(repository: Arc<dyn GetUserRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GetUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<GetUserResponse>;
}
#[async_trait]
impl GetUserService for GetUserServiceImpl {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<GetUserResponse> {
        let Some(user) = self
            .repository
            .find_user(organization_id, application_id, user_id)
            .await?
        else {
            return Err(RepositoryError::new("not found the user".to_string(), 404));
        };
        Ok(GetUserMapper::to_dto(user))
    }
}
