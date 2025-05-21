use std::sync::Arc;

use async_trait::async_trait;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::{
    application::{
        dto::response::refresh_token::CreateTokenResponse,
        repositories::refresh_tokens::{
            create::CreateRefreshTokenRepository, rotate::RotateRefreshTokenRepository,
        },
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
        public_key: String,
    ) -> RepositoryResult<CreateTokenResponse>;
}
#[async_trait]
impl RotateRefreshTokenService for RotateRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        refresh_token: String,
        public_key: String,
    ) -> RepositoryResult<CreateTokenResponse> {
        let dnc_public_key = self.repository.decode_base64(&public_key)?;
        if dnc_public_key.len() != 32 {
            return Err(RepositoryError::new(
                "peseto_key must eq 32".to_string(),
                400,
            ));
        }
        todo!()
    }
}
