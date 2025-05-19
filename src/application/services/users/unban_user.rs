use crate::{
    application::{
        dto::response::user::UnbanUserResponse,
        repositories::users::unban_user::UnbanUserRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UnbanUserServiceImpl {
    pub repository: Arc<dyn UnbanUserRepository>,
}
impl UnbanUserServiceImpl {
    pub fn new(repository: Arc<dyn UnbanUserRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UnbanUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<UnbanUserResponse>;
}
#[async_trait]
impl UnbanUserService for UnbanUserServiceImpl {
    async fn execute(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<UnbanUserResponse> {
        self.repository
            .unban_user(user_id, organization_id, application_id)
            .await?;
        Ok(UnbanUserResponse {
            massage: "success".to_string(),
        })
    }
}
