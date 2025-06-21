use crate::{
    domain::errors::repositories_errors::RepositoryResult,
    infrastructure::repositories::cdc::cdc::CDCExternalRepository,
};
use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

pub struct CDCRepositoryImpl {
    cdc_repo: Arc<dyn CDCExternalRepository>,
}
impl CDCRepositoryImpl {
    pub fn new(cdc_repo: Arc<dyn CDCExternalRepository>) -> Self {
        Self { cdc_repo }
    }
}

#[async_trait]
pub trait CDCInternalRepository: Send + Sync {
    async fn stop(&self, session: Arc<Session>) -> RepositoryResult<()>;
}

#[async_trait]
impl CDCInternalRepository for CDCRepositoryImpl {
    async fn stop(&self, _session: Arc<Session>) -> RepositoryResult<()> {
        Ok(())
    }
}
