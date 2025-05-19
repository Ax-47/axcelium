use crate::{
    application::{
        dto::response::user::DisableMFAUserResponse,
        repositories::users::disable_mfa_user::DisableMFAUserRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DisableMFAUserServiceImpl {
    pub repository: Arc<dyn DisableMFAUserRepository>,
}
impl DisableMFAUserServiceImpl {
    pub fn new(repository: Arc<dyn DisableMFAUserRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DisableMFAUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<DisableMFAUserResponse>;
}
#[async_trait]
impl DisableMFAUserService for DisableMFAUserServiceImpl {
    async fn execute(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<DisableMFAUserResponse> {
        self.repository
            .disable_mfa_user(user_id, organization_id, application_id)
            .await?;
        Ok(DisableMFAUserResponse {
            massage: "success".to_string(),
        })
    }
}
