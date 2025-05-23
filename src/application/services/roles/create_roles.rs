use crate::{
    application::{
        dto::{payload::role::CreateRolePayload, response::refresh_token::SimpleResponse}, mappers::model::ModelMapper, repositories::roles::create_roles::CreateRoleRepository
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::RepositoryResult,
    },
};
use async_trait::async_trait;
use std::sync::Arc;
#[derive(Clone)]
pub struct CreateRoleServiceImpl {
    pub repository: Arc<dyn CreateRoleRepository>,
}
impl CreateRoleServiceImpl {
    pub fn new(repository: Arc<dyn CreateRoleRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait CreateRoleService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        payload: CreateRolePayload,
    ) -> RepositoryResult<SimpleResponse>;
}
#[async_trait]
impl CreateRoleService for CreateRoleServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        payload: CreateRolePayload,
    ) -> RepositoryResult<SimpleResponse> {
        let role = self.repository.new_role(
            c_apporg.organization_id,
            c_apporg.application_id,
            payload.name,
            payload.description,
            payload.permissions,
        );
        self.repository.create_role(&role.to_entity()).await?;
        Ok(SimpleResponse {
            message: "success".to_string(),
        })
    }
}
