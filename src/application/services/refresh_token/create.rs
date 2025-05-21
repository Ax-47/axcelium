use std::sync::Arc;

use async_trait::async_trait;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::{
    application::repositories::refresh_tokens::create::CreateRefreshTokenRepository,
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
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
        paseto_key: String,
    ) -> RepositoryResult<String>;
}
#[async_trait]
impl CreateRefreshTokenService for CreateRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
        paseto_key: String,
    ) -> RepositoryResult<String> {
        let token_secret = self.repository.genarate_token_secret().await?;
        let (secret_key, encrypted_token_secret) = self
            .repository
            .encode_refresh_token_secret(&token_secret)
            .await?;
        let token_version = self.repository.genarate_token_version_base64().await?;

        let issued_at = time::OffsetDateTime::now_utc();
        let expires_at = time::OffsetDateTime::now_utc() + time::Duration::days(30);
        let not_before = time::OffsetDateTime::now_utc() + time::Duration::minutes(40);
        let refresh_token = self.repository.create_refresh_token(
            c_apporg.application_id,
            c_apporg.organization_id,
            user_id,
            encrypted_token_secret,
            token_version,
            issued_at,
            expires_at,
        );
        let paseto_token = self
            .repository
            .create_pesato_token(
                paseto_key,
                refresh_token,
                self.repository.encode_base64(&token_secret).as_str(),
                &secret_key,
                issued_at.format(&Rfc3339)?,
                expires_at.format(&Rfc3339)?,
                not_before.format(&Rfc3339)?,
            )
            .await?;
        Ok(paseto_token)
    }
}
