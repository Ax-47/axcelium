use async_trait::async_trait;

use crate::infrastructure::repositories::hello_repositories::HelloRepository;
use std::sync::Arc;
#[derive(Clone)]
pub struct HelloServiceImpl {
    pub repository: Arc<dyn HelloRepository>,
}
impl HelloServiceImpl {
    pub fn new(repository: Arc<dyn HelloRepository>) -> Self {
        HelloServiceImpl { repository }
    }
}
#[async_trait]
pub trait HelloService: 'static + Sync + Send {
    async fn hello_world(&self) -> String;
}
#[async_trait]
impl HelloService for HelloServiceImpl {
    async fn hello_world(&self) -> String {
        self.repository.hello_world().await
    }
}
