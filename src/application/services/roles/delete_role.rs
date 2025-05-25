use crate::{
    application::{
        dto::response::refresh_token::SimpleResponse,
        repositories::roles::delete_role::DeleteRoleRepository,
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
pub struct DeleteRoleServiceImpl {
    pub repository: Arc<dyn DeleteRoleRepository>,
}
impl DeleteRoleServiceImpl {
    pub fn new(repository: Arc<dyn DeleteRoleRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait DeleteRoleService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
    ) -> RepositoryResult<SimpleResponse>;
}
#[async_trait]
impl DeleteRoleService for DeleteRoleServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
    ) -> RepositoryResult<SimpleResponse> {
        self.repository
            .delete_role(c_apporg.organization_id, c_apporg.application_id, role_id)
            .await?;
        Ok(SimpleResponse {
            message: "success".to_string(),
        })
    }
}
