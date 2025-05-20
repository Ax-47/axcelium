use crate::domain::errors::repositories_errors::RepositoryResult;
use crate::infrastructure::models::refresh_token::RefreshTokenModel;
use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use uuid::Uuid;

pub struct RefreshTokenCacheImpl {
    cache: Arc<Client>,
    ttl: u64,
}

impl RefreshTokenCacheImpl {
    pub fn new(cache: Arc<Client>, ttl: u64) -> Self {
        Self { cache, ttl }
    }
}

#[async_trait]
pub trait RefreshTokenCacheRepository: Send + Sync {
    async fn cache_refresh_token(&self, token: &RefreshTokenModel) -> RepositoryResult<()>;
    async fn get_cached_refresh_token(&self, token_id: Uuid)
        -> RepositoryResult<Option<RefreshTokenModel>>;
    async fn invalidate_refresh_token(&self, token_id: Uuid) -> RepositoryResult<()>;
}

#[async_trait]
impl RefreshTokenCacheRepository for RefreshTokenCacheImpl {
    async fn cache_refresh_token(&self, token: &RefreshTokenModel) -> RepositoryResult<()> {
        let mut conn = self.cache.get_multiplexed_tokio_connection().await?;
        let key = format!("refresh_token:{}", token.token_id);
        let value = serde_json::to_string(token)?;
        let _: () = conn.set_ex(key, value, self.ttl).await?;
        Ok(())
    }

    async fn get_cached_refresh_token(
        &self,
        token_id: Uuid,
    ) -> RepositoryResult<Option<RefreshTokenModel>> {
        let mut conn = self.cache.get_multiplexed_tokio_connection().await?;
        let key = format!("refresh_token:{}", token_id);
        let result: Option<String> = conn.get(key).await?;
        match result {
            Some(json) => {
                let token = serde_json::from_str(&json)?;
                Ok(Some(token))
            }
            None => Ok(None),
        }
    }

    async fn invalidate_refresh_token(&self, token_id: Uuid) -> RepositoryResult<()> {
        let mut conn = self.cache.get_multiplexed_tokio_connection().await?;
        let key = format!("refresh_token:{}", token_id);
        let _: () = conn.del(key).await?;
        Ok(())
    }
}