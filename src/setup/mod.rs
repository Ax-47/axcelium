use crate::{
    application::{
        middlewares::bearer_auth::ValidateBearerAuth,
        services::{
            hello_service::HelloService,
            users::{
                create::CreateUserService, get_user::GetUserService, get_users::GetUsersService,
            },
        },
    },
    config,
};
use redis::Client;
use scylla::client::session::Session;
use std::sync::Arc;
mod middlewares;
mod repositories;
mod services;
pub struct Container {
    pub hello_service: Arc<dyn HelloService>,
    pub create_user_service: Arc<dyn CreateUserService>,
    pub get_users_service: Arc<dyn GetUsersService>,
    pub get_user_service: Arc<dyn GetUserService>,
    pub validate_bearer_auth_middleware_service: Arc<ValidateBearerAuth>,
}

impl Container {
    pub async fn new(cfg: config::Config, cache: Arc<Client>, database: Arc<Session>) -> Self {
        let secret = cfg.core.secret.clone();
        let cache_ttl = cfg.core.cache_ttl.clone();
        let (repos, core_service) =
            repositories::create_all(database.clone(), cache, &secret, cache_ttl);

        core_service.lunch(cfg).await;

        let hello_service = services::create_hello_service();
        let create_user_service = services::create_create_user_service(&repos);
        let get_users_service = services::create_get_users_service(&repos);

        let get_user_service = services::create_get_user_service(&repos);
        let validate_bearer_auth_middleware_service = Arc::new(ValidateBearerAuth::new(
            middlewares::create_validate_bearer_auth_service(&repos),
        ));
        Self {
            hello_service,
            create_user_service,
            get_users_service,
            get_user_service,
            validate_bearer_auth_middleware_service,
        }
    }
}
