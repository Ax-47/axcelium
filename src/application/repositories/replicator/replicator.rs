use crate::infrastructure::repositories::cipher::base64_repository::Base64Repository;
use crate::infrastructure::repositories::database::refresh_token::RefreshTokenDatabaseRepository;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ReplicatorRepositoryImpl {
    database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
    base64_repo: Arc<dyn Base64Repository>,
}
impl ReplicatorRepositoryImpl {
    pub fn new(
        database_repo: Arc<dyn RefreshTokenDatabaseRepository>,
        base64_repo: Arc<dyn Base64Repository>,
    ) -> Self {
        Self {
            database_repo,
            base64_repo,
        }
    }
}

#[async_trait]
pub trait ReplicatorRepository: Send + Sync {}

#[async_trait]
impl ReplicatorRepository for ReplicatorRepositoryImpl {}
