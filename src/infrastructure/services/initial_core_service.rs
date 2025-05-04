use crate::infrastructure::repositories::initial_core::InitialCoreRepository;
use async_trait::async_trait;
use std::sync::Arc;
#[derive(Clone)]
pub struct InitialCoreServiceImpl {
    pub repository: Arc<dyn InitialCoreRepository>,
}
impl InitialCoreServiceImpl {
    pub fn new(repository: Arc<dyn InitialCoreRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
pub trait InitialCoreService: 'static + Sync + Send {
    async fn lunch(&self, act: bool);
}
#[async_trait]
impl InitialCoreService for InitialCoreServiceImpl {
    async fn lunch(&self, act: bool) {
        if !act {
            return;
        }
    }
}
