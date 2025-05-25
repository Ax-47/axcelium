use crate::{
    application::{
        dto::{payload::role::UpdateRolePayload, response::refresh_token::SimpleResponse},
        repositories::roles::update_role::UpdateRoleRepository,
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
pub struct UpdateRoleServiceImpl {
    pub repository: Arc<dyn UpdateRoleRepository>,
}
impl UpdateRoleServiceImpl {
    pub fn new(repository: Arc<dyn UpdateRoleRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait UpdateRoleService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
        payload: UpdateRolePayload,
    ) -> RepositoryResult<SimpleResponse>;
}
#[async_trait]
impl UpdateRoleService for UpdateRoleServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
        payload: UpdateRolePayload,
    ) -> RepositoryResult<SimpleResponse> {
        let Some(fetched_role) = self
            .repository
            .get_role(c_apporg.organization_id, c_apporg.application_id, role_id)
            .await?
        else {
            return Err(RepositoryError {
                message: "not found".to_string(),
                code: 404,
            });
        };
        let mut update_model = self.repository.new_model(
            c_apporg.organization_id,
            c_apporg.application_id,
            role_id,
            fetched_role.name,
            fetched_role.description,
            fetched_role.permissions,
        );
        if let Some(name) = payload.name {
            update_model.name = name;
        }
        update_model.description = payload.description;
        if let Some(permissions) = payload.permissions {
            update_model.permissions = permissions;
        }
        self.repository.update_role(&update_model).await?;
        Ok(SimpleResponse {
            message: "success".to_string(),
        })
    }
}
