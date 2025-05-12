use crate::{
    application::{
        dto::payload::user::UpdateUserPayload,
        repositories::users::update_user::UpdateUserRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
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
    ) -> RepositoryResult<()>;
}
#[async_trait]
impl UpdateUserService for UpdateUserServiceImpl {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
        update: UpdateUserPayload,
    ) -> RepositoryResult<()> {
        println!("test");
        self.repository
            .update_user(organization_id, application_id, user_id, update)
            .await?;
        Ok(())
    }
}
