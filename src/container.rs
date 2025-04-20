use sqlx::MySqlPool;

use crate::infrastructure::{
    repositories::{hello_repositories::{HelloRepository, HelloRepositoryImpl}, user_repositories::{UserRepository, UserRepositoryImpl}},

    services::{hello_service::{HelloService, HelloServiceImpl}, user_service::{ UserService, UserServiceImpl}},
};
use std::sync::Arc;
use redis::Client as RedisClient;

pub struct Container {
    pub hello_service: Arc<dyn HelloService>,
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub fn new(cache: Arc<RedisClient>, database: Arc<MySqlPool>) -> Self {
        let hello_repository: Arc<dyn HelloRepository> = Arc::new(HelloRepositoryImpl::new());
        let hello_service = Arc::new(HelloServiceImpl {
            repository: hello_repository,
        });
        let user_repository: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(cache,database));
        let user_service = Arc::new(UserServiceImpl{
            repository: user_repository,
        });
        Container { hello_service, user_service }
    }
}

