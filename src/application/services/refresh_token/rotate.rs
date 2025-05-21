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
pub struct CreateRefreshTokenServiceImpl {
    pub repository: Arc<dyn RotateRefreshTokenRepository>,
}
impl CreateRefreshTokenServiceImpl {
    pub fn new(repository: Arc<dyn RotateRefreshTokenRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait CreateRefreshTokenService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        refresh_token: String,
    ) -> RepositoryResult<CreateTokenResponse>;
}
#[async_trait]
impl CreateRefreshTokenService for CreateRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        refresh_token: String,
    ) -> RepositoryResult<CreateTokenResponse> {
        todo!()
    }
}
