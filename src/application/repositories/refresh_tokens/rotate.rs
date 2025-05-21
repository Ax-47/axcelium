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
    async fn encode_refresh_token_secret(
        &self,
        encrypt_client_secret: &Vec<u8>,
    ) -> RepositoryResult<(String, String)>;

    async fn genarate_token_secret(&self) -> RepositoryResult<Vec<u8>>;
    async fn genarate_token_version_base64(&self) -> RepositoryResult<String>;
    fn create_refresh_token(
        &self,
        application_id: Uuid,
        organization_id: Uuid,
        user_id: Uuid,
        encrypted_token_secret: String,
        token_version: String,
        issued_at: OffsetDateTime,
        expires_at: OffsetDateTime,
    ) -> RefreshToken;

    async fn create_pesato_token(
        &self,
        key: &Vec<u8>,
        rt: RefreshToken,
        secret: &str,
        secret_key: &str,
        issued_at: String,
        expire: String,
        notbefore: String,
    ) -> RepositoryResult<String>;

    async fn encrypt_paseto(&self, rt: String, pk: String);
    fn encode_base64(&self, bytes: &Vec<u8>) -> String;

    fn decode_base64(&self, plaintext: &str) -> RepositoryResult<Vec<u8>>;

    async fn store_refresh_token(&self, rf: RefreshToken) -> RepositoryResult<()>;
}

#[async_trait]
impl RotateRefreshTokenRepository for RotateRefreshTokenRepositoryImpl {
    async fn genarate_token_secret(&self) -> RepositoryResult<Vec<u8>> {
        let mut secret = vec![0u8; 32];
        OsRng.try_fill_bytes(&mut secret)?;
        Ok(secret)
    }
    async fn genarate_token_version_base64(&self) -> RepositoryResult<String> {
        let mut version = vec![0u8; 24];
        OsRng.try_fill_bytes(&mut version)?;
        Ok(self.base64_repo.encode(&version))
    }
    async fn encode_refresh_token_secret(
        &self,
        client_secret: &Vec<u8>,
    ) -> RepositoryResult<(String, String)> {
        self.aes_repo.encrypt(client_secret).await
    }

    fn encode_base64(&self, bytes: &Vec<u8>) -> String {
        self.base64_repo.encode(bytes)
    }

    fn decode_base64(&self, plaintext: &str) -> RepositoryResult<Vec<u8>> {
        Ok(self.base64_repo.decode(plaintext)?)
    }
    fn create_refresh_token(
        &self,
        application_id: Uuid,
        organization_id: Uuid,
        user_id: Uuid,
        encrypted_token_secret: String,
        token_version: String,
        issued_at: OffsetDateTime,
        expires_at: OffsetDateTime,
    ) -> RefreshToken {
        RefreshToken {
            token_id: Uuid::new_v4(),
            application_id,
            organization_id,
            user_id,
            encrypted_token_secret,
            token_version,
            parent_version: None,
            issued_at: CqlTimestamp(issued_at.unix_timestamp()),
            expires_at: CqlTimestamp(expires_at.unix_timestamp()),
            revoked: false,
        }
    }
    async fn create_pesato_token(
        &self,
        key: &Vec<u8>,
        rt: RefreshToken,
        secret: &str,
        secret_key: &str,
        issued_at: String,
        expire: String,
        notbefore: String,
    ) -> RepositoryResult<String> {
        self.paseto_repo
            .encrypt(key, rt, secret, secret_key, issued_at, expire, notbefore)
            .await
    }
    async fn encrypt_paseto(&self, rt: String, pk: String) {
        self.paseto_repo.decrypt( &rt,&pk).await;

    }
    async fn store_refresh_token(&self, rf: RefreshToken) -> RepositoryResult<()> {
        self.database_repo.create_refresh_token(rf.into()).await
    }
}
