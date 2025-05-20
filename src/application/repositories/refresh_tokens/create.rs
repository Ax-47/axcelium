use crate::domain::entities::refresh_token::RefreshToken;
use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::repositories::cache_layer::refresh_token_repository::RefreshTokenCacheLayerRepository;
use crate::infrastructure::repositories::cipher::{
    aes_gcm_repository::AesGcmCipherRepository, base64_repository::Base64Repository,
};
use async_trait::async_trait;
use rand_core::{OsRng, TryRngCore};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateRefreshTokenRepositoryImpl {
    cache_layer_repo: Arc<dyn RefreshTokenCacheLayerRepository>,
    base64_repo: Arc<dyn Base64Repository>,
    aes_repo: Arc<dyn AesGcmCipherRepository>,
}
impl CreateRefreshTokenRepositoryImpl {
    pub fn new(
        cache_layer_repo: Arc<dyn RefreshTokenCacheLayerRepository>,
        base64_repo: Arc<dyn Base64Repository>,
        aes_repo: Arc<dyn AesGcmCipherRepository>,
    ) -> Self {
        Self {
            cache_layer_repo,
            base64_repo,
            aes_repo,
        }
    }
}

#[async_trait]
pub trait CreateRefreshTokenRepository: Send + Sync {
    async fn encode_refresh_token_secret(
        &self,
        encrypt_client_secret: &Vec<u8>,
    ) -> RepositoryResult<(String, String)>;

    async fn genarate_token_secret(&self) -> RepositoryResult<Vec<u8>>;

    // async fn create_refresh_token(&self);
}

#[async_trait]
impl CreateRefreshTokenRepository for CreateRefreshTokenRepositoryImpl {
    async fn genarate_token_secret(&self) -> RepositoryResult<Vec<u8>> {
        let mut secret = vec![0u8; 32];
        OsRng.try_fill_bytes(&mut secret)?;
        Ok(secret)
    }
    async fn encode_refresh_token_secret(
        &self,
        client_secret: &Vec<u8>,
    ) -> RepositoryResult<(String, String)> {
        self.aes_repo.encrypt(client_secret).await
    }
//     async fn create_refresh_token(&self, application: Uuid, organization_id: Uuid, user_id: Uuid,encrypted_client_secret:&str) {
//         RefreshToken {
//             token_id: Uuid::new_v4(),
//             application_id,
//             organization_id,
//             user_id,
// encrypted_client_secret
//         }
//     }
}
