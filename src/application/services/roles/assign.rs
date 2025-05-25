use crate::{
    application::{
        dto::response::refresh_token::SimpleResponse, repositories::roles::assign::AssignRepository,
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
pub struct AssignServiceImpl {
    pub repository: Arc<dyn AssignRepository>,
}
impl AssignServiceImpl {
    pub fn new(repository: Arc<dyn AssignRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait AssignService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<SimpleResponse>;
}
#[async_trait]
impl AssignService for AssignServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        role_id: Uuid,
        user_id: Uuid,
    ) -> RepositoryResult<SimpleResponse> {
        self.repository
            .assign(
                c_apporg.organization_id,
                c_apporg.application_id,
                role_id,
                user_id,
            )
            .await?;
        Ok(SimpleResponse {
            message: "success".to_string(),
        })
    }
}
