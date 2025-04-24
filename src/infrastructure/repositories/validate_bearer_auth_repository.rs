use async_trait::async_trait;
use redis::Client as RedisClient;
use scylla::client::session::Session;
use std::sync::Arc;
pub struct VaildateBearerAuthMiddlewareRepositoryImpl {
    pub cache: Arc<RedisClient>,
    pub database: Arc<Session>,
}
impl VaildateBearerAuthMiddlewareRepositoryImpl {
    pub fn new(cache: Arc<RedisClient>, database: Arc<Session>) -> Self {
        Self { cache, database }
    }
}

#[async_trait]
pub trait VaildateBearerAuthMiddlewareRepository: Send + Sync {}

#[async_trait]
impl VaildateBearerAuthMiddlewareRepository for VaildateBearerAuthMiddlewareRepositoryImpl {}
