use std::sync::Arc;

use async_trait::async_trait;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::{
    application::{
        dto::response::refresh_token::CreateTokenResponse,
        repositories::refresh_tokens::{create::CreateRefreshTokenRepository, rotate::RotateRefreshTokenRepository},
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::{RepositoryError, RepositoryResult},
    },
};
#[derive(Clone)]
pub struct RotateRefreshTokenServiceImpl {
    pub repository: Arc<dyn RotateRefreshTokenRepository>,
}
impl RotateRefreshTokenServiceImpl {
    pub fn new(repository: Arc<dyn RotateRefreshTokenRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait RotateRefreshTokenService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        refresh_token: String,
    ) -> RepositoryResult<CreateTokenResponse>;
}
#[async_trait]
impl RotateRefreshTokenService for RotateRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        refresh_token: String,
    ) -> RepositoryResult<CreateTokenResponse> {
        todo!()
    }
}
