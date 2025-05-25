use crate::{
    application::{
        dto::response::role::GetRolesByAppResponse,
        repositories::roles::get_roles_by_app::GetRolesByAppRepository,
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::RepositoryResult,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
#[derive(Clone)]
pub struct GetRolesByAppServiceImpl {
    pub repository: Arc<dyn GetRolesByAppRepository>,
}
impl GetRolesByAppServiceImpl {
    pub fn new(repository: Arc<dyn GetRolesByAppRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait GetRolesByAppService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<GetRolesByAppResponse>;
}
#[async_trait]
impl GetRolesByAppService for GetRolesByAppServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
    ) -> RepositoryResult<GetRolesByAppResponse> {
        let res = self
            .repository
            .get_roles_by_app(c_apporg.organization_id, c_apporg.application_id)
            .await?;
        Ok(GetRolesByAppResponse { roles: res })
    }
}
