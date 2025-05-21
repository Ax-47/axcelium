use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
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
                "public_key must eq 32".to_string(),
                400,
            ));
        }
        let token = self
            .repository
            .decrypt_paseto(refresh_token.as_str(), &dnc_public_key)
            .await?;
        let now = Utc::now().timestamp();

        // if token.exp <= now {
        //     return Err(RepositoryError::new("Token has expired".to_string(), 401));
        // }
        // if token.nbf > now {
        //     return Err(RepositoryError::new("Token not valid yet".to_string(), 401));
        // }
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
        println!("{:#?}",fetched_token);
        todo!()
    }
}
