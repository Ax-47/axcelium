use crate::infrastructure::{
    repositories::hello_repositories::{HelloRepository, HelloRepositoryImpl},
    services::hello_service::{HelloService, HelloServiceImpl},
};
use std::sync::Arc;

pub struct Container {
    pub hello_service: Arc<dyn HelloService>,
}

impl Container {
    pub fn new() -> Self {
        let todo_repository: Arc<dyn HelloRepository> = Arc::new(HelloRepositoryImpl::new());
        let hello_service = Arc::new(HelloServiceImpl {
            repository: todo_repository,
        });
        Container { hello_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
