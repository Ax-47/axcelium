use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::repositories::{
    cache_layer::applications_organization_by_client_id_repository::ApplicationsOrganizationByClientIdCacheLayerRepository,
    cipher::{aes_gcm_repository::AesGcmCipherRepository, base64_repository::Base64Repository},
};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CreateRefreshTokenRepositoryImpl {
    base64_repo: Arc<dyn Base64Repository>,
    aes_repo: Arc<dyn AesGcmCipherRepository>,
}
impl CreateRefreshTokenRepositoryImpl {
    pub fn new(
        base64_repo: Arc<dyn Base64Repository>,
        aes_repo: Arc<dyn AesGcmCipherRepository>,
    ) -> Self {
        Self {
            base64_repo,
            aes_repo,
        }
    }
}

#[async_trait]
pub trait CreateRefreshTokenRepository: Send + Sync {
    async fn encode_refresh_token_secret(
        &self,
        client_key: &str,
        encrypt_client_secret: &str,
    ) -> RepositoryResult<String>;
}

#[async_trait]
impl CreateRefreshTokenRepository for CreateRefreshTokenRepositoryImpl {
    async fn encode_refresh_token_secret(
        &self,
        client_key: &str,
        encrypt_client_secret: &str,
    ) -> RepositoryResult<String> {
        todo!()
    }
}
