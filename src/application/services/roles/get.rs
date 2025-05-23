use crate::{
    application::{
        dto::response::refresh_token::SimpleResponse,
        repositories::roles::create_roles::CreateRoleRepository,
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
pub struct GetRoleServiceImpl {
    pub repository: Arc<dyn CreateRoleRepository>,
}
impl GetRoleServiceImpl {
    pub fn new(repository: Arc<dyn CreateRoleRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait GetRoleService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role: Uuid,
    ) -> RepositoryResult<SimpleResponse>;
}
#[async_trait]
impl GetRoleService for GetRoleServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
    ) -> RepositoryResult<SimpleResponse> {
        self.repository
            .get_role(c_apporg.organization_id, c_apporg.application_id, role_id)
            .await?;
        Ok(SimpleResponse {
            message: "success".to_string(),
        })
    }
}
