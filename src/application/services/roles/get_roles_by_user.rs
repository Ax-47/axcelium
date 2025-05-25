use crate::{
    application::{
        dto::response::role::GetRolesByUserResponse,
        repositories::roles::get_roles_by_user::GetRolesByUserRepository,
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::RepositoryResult,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone)]
pub struct GetRolesByUserServiceImpl {
    pub repository: Arc<dyn GetRolesByUserRepository>,
}
impl GetRolesByUserServiceImpl {
    pub fn new(repository: Arc<dyn GetRolesByUserRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait GetRolesByUserService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
    ) -> RepositoryResult<GetRolesByUserResponse>;
}
#[async_trait]
impl GetRolesByUserService for GetRolesByUserServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
    ) -> RepositoryResult<GetRolesByUserResponse> {
        let res = self
            .repository
            .get_roles_by_user(c_apporg.organization_id, c_apporg.application_id, user_id)
            .await?;
        Ok(GetRolesByUserResponse { roles: res })
    }
}
