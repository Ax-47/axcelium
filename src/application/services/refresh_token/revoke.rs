use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::{
        dto::response::refresh_token::CreateTokenResponse,
        repositories::refresh_tokens::revoke::RevokeRefreshTokenRepository,
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::RepositoryResult,
    },
};
#[derive(Clone)]
pub struct RevokeRefreshTokenServiceImpl {
    pub repository: Arc<dyn RevokeRefreshTokenRepository>,
}
impl RevokeRefreshTokenServiceImpl {
    pub fn new(repository: Arc<dyn RevokeRefreshTokenRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait RevokeRefreshTokenService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        token_id: Uuid,
    ) -> RepositoryResult<CreateTokenResponse>;
}
#[async_trait]
impl RevokeRefreshTokenService for RevokeRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        token_id: Uuid,
    ) -> RepositoryResult<CreateTokenResponse> {
        self.repository
            .revoke_refresh_token(c_apporg.organization_id, c_apporg.application_id, token_id)
            .await?;
        todo!()
    }
}
