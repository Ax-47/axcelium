use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::repositories::refresh_tokens::create::CreateRefreshTokenRepository,
    domain::{
        entities::{apporg_client_id::CleanAppOrgByClientId, refresh_token::RefreshToken},
        errors::repositories_errors::RepositoryResult,
    },
};
#[derive(Clone)]
pub struct CreateRefreshTokenServiceImpl {
    pub repository: Arc<dyn CreateRefreshTokenRepository>,
}
impl CreateRefreshTokenServiceImpl {
    pub fn new(repository: Arc<dyn CreateRefreshTokenRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait CreateRefreshTokenService: 'static + Sync + Send {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
    ) -> RepositoryResult<String>;
}
#[async_trait]
impl CreateRefreshTokenService for CreateRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
    ) -> RepositoryResult<String> {
        let client_secret = self.repository.genarate_token_secret().await?;
        let (secret_key, encrypted_client_secret) = self
            .repository
            .encode_refresh_token_secret(&client_secret)
            .await?;
        todo!()
    }
}
