use std::sync::Arc;

use crate::application::{
    services::hello_service::{HelloService, HelloServiceImpl},
    services::users::create::{CreateUserService, CreateUserServiceImpl},
};

use super::repositories::Repositories;

pub fn create_hello_service() -> Arc<dyn HelloService> {
    Arc::new(HelloServiceImpl {})
}

pub fn create_user_service(repos: &Repositories) -> Arc<dyn CreateUserService> {
    Arc::new(CreateUserServiceImpl {
        repository: repos.user_repo.clone(),
    })
}
