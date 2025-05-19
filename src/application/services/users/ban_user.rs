use crate::{
    application::{
        dto::response::user::BanUserResponse, repositories::users::ban_user::BanUserRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct BanUserServiceImpl {
    pub repository: Arc<dyn BanUserRepository>,
}
impl BanUserServiceImpl {
    pub fn new(repository: Arc<dyn BanUserRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BanUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<BanUserResponse>;
}
#[async_trait]
impl BanUserService for BanUserServiceImpl {
    async fn execute(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        application_id: Uuid,
    ) -> RepositoryResult<BanUserResponse> {
        self.repository
            .ban_user(user_id, organization_id, application_id)
            .await?;
        Ok(BanUserResponse {
            massage: "success".to_string(),
        })
    }
}
