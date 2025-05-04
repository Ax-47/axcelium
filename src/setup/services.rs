use std::sync::Arc;

use crate::infrastructure::{
    services::hello_service::{HelloService, HelloServiceImpl},
    services::user_service::{UserService, UserServiceImpl},
};

use super::repositories::Repositories;

pub fn create_hello_service() -> Arc<dyn HelloService> {
    Arc::new(HelloServiceImpl {})
}

pub fn create_user_service(repos: &Repositories) -> Arc<dyn UserService> {
    Arc::new(UserServiceImpl {
        repository: repos.user_repo.clone(),
    })
}
