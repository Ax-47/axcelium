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
    infrastructure::repositories::paseto::PASETO_V4_LOCAL_KEY_LEN,
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
        let old_token_id = Uuid::parse_str(token.jti.as_str())?;
        let Some(fetched_token) = self
            .repository
            .find_refresh_token(
                c_apporg.organization_id,
                c_apporg.application_id,
                old_token_id,
                &token.version,
            )
            .await?
        else {
            return Err(RepositoryError::new(
                "Refresh token not found for the given organization_id and application_id"
                    .to_string(),
                400,
            ));
        };
        if fetched_token.revoked {
            return Err(RepositoryError::new(
                "the refresh token was revoked".to_string(),
                400,
            ));
        }
        //issue new token
        let token_version = self.repository.genarate_token_version_base64().await?;
        let issued_at = time::OffsetDateTime::now_utc();
        let expires_at = issued_at + time::Duration::days(30);
        let not_before = issued_at + time::Duration::minutes(40);
        let refresh_token = self.repository.create_refresh_token(
            old_token_id,
            c_apporg.application_id,
            c_apporg.organization_id,
            fetched_token.user_id,
            fetched_token.encrypted_token_secret,
            token_version,
            token.version,
            issued_at,
            expires_at,
        );
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
        self.repository.update_refresh_token(&refresh_token).await?;
        let paseto_token = self
            .repository
            .create_pesato_token(
                &dnc_private_key,
                refresh_token,
                &token.secret,
                &token.secret_key,
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
