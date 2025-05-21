use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::{
    application::{
        dto::response::refresh_token::CreateTokenResponse,
        repositories::refresh_tokens::rotate::RotateRefreshTokenRepository,
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
        private_key: String,
    ) -> RepositoryResult<CreateTokenResponse>;
}
#[async_trait]
impl RotateRefreshTokenService for RotateRefreshTokenServiceImpl {
    async fn execute(
        &self,
        c_apporg: CleanAppOrgByClientId,
        refresh_token: String,
        public_key: String,
        private_key: String,
    ) -> RepositoryResult<CreateTokenResponse> {
        let dnc_public_key = self.repository.decode_base64(&public_key)?;
        if dnc_public_key.len() != 32 {
            return Err(RepositoryError::new(
                "public_key must eq 32".to_string(),
                400,
            ));
        }
        let token = self
            .repository
            .decrypt_paseto(refresh_token.as_str(), &dnc_public_key)
            .await?;
        let now = Utc::now().timestamp();

        if token.exp <= now {
            return Err(RepositoryError::new("Token has expired".to_string(), 401));
        }
        if token.nbf > now {
            return Err(RepositoryError::new("Token not valid yet".to_string(), 401));
        }
        let Some(fetched_token) = self
            .repository
            .find_refresh_token(
                c_apporg.organization_id,
                c_apporg.application_id,
                Uuid::parse_str(token.jti.as_str())?,
            )
            .await?
        else {
            return Err(RepositoryError::new("not found".to_string(), 400));
        };
        if fetched_token.revoked {
            return Err(RepositoryError::new(
                "the refresh token was revoked".to_string(),
                400,
            ));
        }
        //issue new token
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
            fetched_token.user_id,
            encrypted_token_secret,
            token_version,
            token.version,
            issued_at,
            expires_at,
        );
        self.repository
            .store_refresh_token(refresh_token.clone())
            .await?;
        let dnc_private_key = self.repository.decode_base64(&private_key)?;
        if dnc_private_key.len() != 64 {
            return Err(RepositoryError::new(
                "peseto_key must eq 64".to_string(),
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
                not_before.format(&Rfc3339)?,
            )
            .await?;
        Ok(CreateTokenResponse {
            refresh_token: paseto_token,
        })
    }
}
