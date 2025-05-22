use crate::{
    application::{
        dto::response::refresh_token::CreateTokenResponse,
        repositories::refresh_tokens::create::CreateRefreshTokenRepository,
    },
    domain::{
        entities::apporg_client_id::CleanAppOrgByClientId,
        errors::repositories_errors::{RepositoryError, RepositoryResult},
    }, infrastructure::repositories::paseto::PASETO_V4_LOCAL_KEY_LEN,
};
use async_trait::async_trait;
use std::sync::Arc;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;
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
        private_key: String,
    ) -> RepositoryResult<CreateTokenResponse>;
}
#[async_trait]
impl CreateRefreshTokenService for CreateRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        user_id: Uuid,
        private_key: String,
    ) -> RepositoryResult<CreateTokenResponse> {
        let token_secret = self.repository.generate_token_secret().await?;
        let (secret_key, encrypted_token_secret) = self
            .repository
            .encode_refresh_token_secret(&token_secret)
            .await?;
        let token_version = self.repository.generate_token_version_base64().await?;

        let issued_at = time::OffsetDateTime::now_utc();
        let expires_at = issued_at + time::Duration::days(30);
        let not_before = issued_at + time::Duration::minutes(40);
        let refresh_token = self.repository.create_refresh_token(
            c_apporg.application_id,
            c_apporg.organization_id,
            user_id,
            encrypted_token_secret,
            token_version,
            issued_at,
            expires_at,
        );
        self.repository.store_refresh_token(&refresh_token).await?;
        let dnc_private_key = self.repository.decode_base64(&private_key)?;
        if dnc_private_key.len() != PASETO_V4_LOCAL_KEY_LEN {
            return Err(RepositoryError::new(
                format!(
                    "Invalid private_key length: expected {}, got {}",
                    PASETO_V4_LOCAL_KEY_LEN,
                    dnc_private_key.len()
                ),
                400,
            ));
        }

        let paseto_token = self
            .repository
            .create_pesato_token(
                &dnc_private_key,
                refresh_token,
                self.repository.encode_base64(&token_secret).as_str(),
                &secret_key,
                issued_at.format(&Rfc3339)?,
                expires_at.format(&Rfc3339)?,
                not_before.format(&Rfc3339)?, //not before
            )
            .await?;
        Ok(CreateTokenResponse {
            refresh_token: paseto_token,
        })
    }
}
