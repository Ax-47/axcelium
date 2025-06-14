use crate::application::repositories::replicator::replicator::ReplicatorRepository;
use crate::domain::errors::repositories_errors::RepositoryResult;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ReplicatorServiceImpl {
    pub repository: Arc<dyn ReplicatorRepository>,
}
impl ReplicatorServiceImpl {
    pub fn new(repository: Arc<dyn ReplicatorRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait ReplicatorService: Send + Sync {
    async fn execute(&self) -> RepositoryResult<()>;
}

#[async_trait]
impl ReplicatorService for ReplicatorServiceImpl {
    async fn execute(&self) -> RepositoryResult<()> {
        Ok(())
    }
}
