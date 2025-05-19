use crate::{
    application::{
        dto::{payload::user::UpdateUserPayload, response::user::UpdateUsersResponse},
        repositories::users::update_user::UpdateUserRepository,
    },
    domain::errors::repositories_errors::{RepositoryError, RepositoryResult},
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UpdateUserServiceImpl {
    pub repository: Arc<dyn UpdateUserRepository>,
}
impl UpdateUserServiceImpl {
    pub fn new(repository: Arc<dyn UpdateUserRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UpdateUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        update: UpdateUserPayload,
    ) -> RepositoryResult<UpdateUsersResponse>;
}
#[async_trait]
impl UpdateUserService for UpdateUserServiceImpl {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        mut update: UpdateUserPayload,
    ) -> RepositoryResult<UpdateUsersResponse> {
        if self
            .repository
            .find_user(organization_id, application_id, user_id)
            .await?
            .is_none()
        {
            return Err(RepositoryError::new("not found user".to_string(), 400));
        };
        if let Some(password) = update.password {
            update.password = Some(self.repository.hash_password(password)?);
        }
        self.repository
            .update_user(organization_id, application_id, user_id, update)
            .await?;
        Ok(UpdateUsersResponse {
            massage: "success".to_string(),
        })
    }
}
