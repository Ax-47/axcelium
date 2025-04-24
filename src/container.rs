use crate::infrastructure::{
    repositories::{
        hello_repository::{HelloRepository, HelloRepositoryImpl},
        user_repository::{UserRepository, UserRepositoryImpl},
        validate_bearer_auth_repository::{
            VaildateBearerAuthMiddlewareRepository, VaildateBearerAuthMiddlewareRepositoryImpl,
        },
    },
    services::{
        hello_service::{HelloService, HelloServiceImpl},
        user_service::{UserService, UserServiceImpl},
        validate_bearer_auth_service::{
            VaildateBearerAuthMiddlewareService, VaildateBearerAuthMiddlewareServiceImpl,
        },
    },
};
use redis::Client as RedisClient;
use scylla::client::session::Session;
use std::sync::Arc;

pub struct Container {
    pub hello_service: Arc<dyn HelloService>,
    pub user_service: Arc<dyn UserService>,
    pub validate_bearer_auth_middleware_service: Arc<dyn VaildateBearerAuthMiddlewareService>,
}

impl Container {
    pub fn new(cache: Arc<RedisClient>, database: Arc<Session>) -> Self {
        //controllers
        let hello_repository: Arc<dyn HelloRepository> = Arc::new(HelloRepositoryImpl::new());
        let hello_service = Arc::new(HelloServiceImpl {
            repository: hello_repository,
        });
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(UserRepositoryImpl::new(cache.clone(), database.clone()));
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repository,
        });
        //middlewares
        let validate_bearer_auth_middleware_repository: Arc<
            dyn VaildateBearerAuthMiddlewareRepository,
        > = Arc::new(VaildateBearerAuthMiddlewareRepositoryImpl::new(
            cache, database,
        ));
        let validate_bearer_auth_middleware_service =
            Arc::new(VaildateBearerAuthMiddlewareServiceImpl {
                repository: validate_bearer_auth_middleware_repository,
            });
        Container {
            hello_service,
            user_service,
            validate_bearer_auth_middleware_service,
        }
    }
}
