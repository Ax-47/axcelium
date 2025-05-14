use crate::{
    application::{
        dto::response::user::UpdateUsersResponse, repositories::users::delete::DeleteUserRepository,
    },
    domain::errors::repositories_errors::RepositoryResult,
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeleteUserServiceImpl {
    pub repository: Arc<dyn DeleteUserRepository>,
}
impl DeleteUserServiceImpl {
    pub fn new(repository: Arc<dyn DeleteUserRepository>) -> Self {
        Self { repository }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DeleteUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<UpdateUsersResponse>;
}
#[async_trait]
impl DeleteUserService for DeleteUserServiceImpl {
    async fn execute(
        &self,
        organization_id: Uuid,
        application_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<UpdateUsersResponse> {
        self.repository
            .delete_user(organization_id, application_id, user_id)
            .await?;
        Ok(UpdateUsersResponse {
            massage: "success".to_string(),
        })
    }
}
