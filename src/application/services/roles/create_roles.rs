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
        token_id: Uuid,
    ) -> RepositoryResult<SimpleResponse>;
}
#[async_trait]
impl CreateRoleService for CreateRoleServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        token_id: Uuid,
    ) -> RepositoryResult<SimpleResponse> {
        Ok(SimpleResponse {
            message: "success".to_string(),
        })
    }
}
