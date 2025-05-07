use std::sync::Arc;

use crate::application::services::{hello_service::{HelloService, HelloServiceImpl}, users::{create::{CreateUserService, CreateUserServiceImpl}, get_users::{GetUsersService, GetUsersServiceImpl}}};

use super::repositories::Repositories;

pub fn create_hello_service() -> Arc<dyn HelloService> {
    Arc::new(HelloServiceImpl {})
}

pub fn create_create_user_service(repos: &Repositories) -> Arc<dyn CreateUserService> {
    Arc::new(CreateUserServiceImpl {
        repository: repos.create_user_repo.clone(),
    })
}

pub fn create_get_users_service(repos: &Repositories) -> Arc<dyn GetUsersService> {
    Arc::new(GetUsersServiceImpl {
        repository: repos.get_users_repo.clone(),
    })
}