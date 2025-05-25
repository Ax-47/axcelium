use crate::{
    application::{
        dto::response::role::GetRoleResponse,
        repositories::roles::get_role_by_app::GetRoleByAppRepository,
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::{RepositoryError, RepositoryResult},
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone)]
pub struct GetRoleByAppServiceImpl {
    pub repository: Arc<dyn GetRoleByAppRepository>,
}
impl GetRoleByAppServiceImpl {
    pub fn new(repository: Arc<dyn GetRoleByAppRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait GetRoleByAppService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
    ) -> RepositoryResult<GetRoleResponse>;
}
#[async_trait]
impl GetRoleByAppService for GetRoleByAppServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
    ) -> RepositoryResult<GetRoleResponse> {
        let Some(res) = self
            .repository
            .get_role(c_apporg.organization_id, c_apporg.application_id, role_id)
            .await?
        else {
            return Err(RepositoryError {
                message: "not found".to_string(),
                code: 404,
            });
        };
        Ok(GetRoleResponse {
            name: res.name,
            description: res.description,
            permissions: res.permissions,
            created_at: res.created_at,
            updated_at: res.updated_at,
        })
    }
}
