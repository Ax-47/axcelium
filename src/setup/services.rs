use std::sync::Arc;

use crate::application::services::{
    hello_service::{HelloService, HelloServiceImpl},
    users::{
        create::{CreateUserService, CreateUserServiceImpl}, delete::{DeleteUserService, DeleteUserServiceImpl}, get_user::{GetUserService, GetUserServiceImpl}, get_users::{GetUsersService, GetUsersServiceImpl}, update_user::{UpdateUserService, UpdateUserServiceImpl}
    },
};

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
pub fn create_get_user_service(repos: &Repositories) -> Arc<dyn GetUserService> {
    Arc::new(GetUserServiceImpl {
        repository: repos.get_user_repo.clone(),
    })
}

pub fn create_update_user_service(repos: &Repositories) -> Arc<dyn UpdateUserService> {
    Arc::new(UpdateUserServiceImpl {
        repository: repos.update_user_repo.clone(),
    })
}

pub fn create_delete_user_service(repos: &Repositories) -> Arc<dyn DeleteUserService> {
    Arc::new(DeleteUserServiceImpl {
        repository: repos.del_user_repo.clone(),
    })
}