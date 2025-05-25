use crate::{
    application::{
        dto::response::role::GetUsersByRoleResponse,
        repositories::roles::get_users_by_role::GetUsersByRoleRepository,
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
pub struct GetUsersByRoleServiceImpl {
    pub repository: Arc<dyn GetUsersByRoleRepository>,
}
impl GetUsersByRoleServiceImpl {
    pub fn new(repository: Arc<dyn GetUsersByRoleRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait GetUsersByRoleService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
    ) -> RepositoryResult<GetUsersByRoleResponse>;
}
#[async_trait]
impl GetUsersByRoleService for GetUsersByRoleServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
    ) -> RepositoryResult<GetUsersByRoleResponse> {
        let users = self
            .repository
            .get_users_by_role(c_apporg.organization_id, c_apporg.application_id, role_id)
            .await?;
        Ok(GetUsersByRoleResponse { users })
    }
}
