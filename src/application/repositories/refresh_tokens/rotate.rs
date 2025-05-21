use crate::domain::entities::refresh_token::RefreshToken;
use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::repositories::cipher::{
    aes_gcm_repository::AesGcmCipherRepository, base64_repository::Base64Repository,
};
use crate::infrastructure::repositories::database::refresh_token::RefreshTokenDatabaseRepository;
use crate::infrastructure::repositories::paseto::refresh_token::PasetoRepository;
use async_trait::async_trait;
use rand_core::{OsRng, TryRngCore};
use scylla::value::CqlTimestamp;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct RotateRefreshTokenRepositoryImpl {
    paseto_repo: Arc<dyn PasetoRepository>,
    database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
    base64_repo: Arc<dyn Base64Repository>,
    aes_repo: Arc<dyn AesGcmCipherRepository>,
}
impl RotateRefreshTokenRepositoryImpl {
    pub fn new(
        paseto_repo: Arc<dyn PasetoRepository>,
        database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
        base64_repo: Arc<dyn Base64Repository>,
        aes_repo: Arc<dyn AesGcmCipherRepository>,
    ) -> Self {
        Self {
            paseto_repo,
            database_repo,
            base64_repo,
            aes_repo,
        }
    }
}

#[async_trait]
pub trait RotateRefreshTokenRepository: Send + Sync {
}

#[async_trait]
impl RotateRefreshTokenRepository for RotateRefreshTokenRepositoryImpl {
}
