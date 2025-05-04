use crate::{
    application::middlewares::bearer_auth::ValidateBearerAuth,
    infrastructure::services::{hello_service::HelloService, user_service::UserService},
};
use redis::cluster::ClusterClient;
use scylla::client::session::Session;
use std::{env, sync::Arc};
mod middlewares;
mod repositories;
mod services;
pub struct Container {
    pub hello_service: Arc<dyn HelloService>,
    pub user_service: Arc<dyn UserService>,
    pub validate_bearer_auth_middleware_service: Arc<ValidateBearerAuth>,
}

impl Container {
    fn get_env(key: &str) -> String {
        env::var(key).unwrap()
    }

    fn get_env_bool(key: &str) -> bool {
        env::var(key).map(|v| v == "true").unwrap_or(false)
    }

    fn get_env_u64(key: &str) -> u64 {
        env::var(key).map(|u| u.parse::<u64>().unwrap()).unwrap()
    }

    pub async fn new(cache: Arc<ClusterClient>, database: Arc<Session>) -> Self {
        let secret = Self::get_env("CORE_SECRET");
        let cache_ttl = Self::get_env_u64("APPLICATIONS_ORGANIZATION_CACHE_TTL");
        let do_gen_core = Self::get_env_bool("CORE_GENRATE_CORE_ORG_APP");

        let (repos, core_service) =
            repositories::create_all(database.clone(), cache, &secret, cache_ttl);

        core_service.lunch(do_gen_core).await;

        let hello_service = services::create_hello_service();
        let user_service = services::create_user_service(&repos);

        let validate_bearer_auth_middleware_service = Arc::new(ValidateBearerAuth::new(
            middlewares::create_validate_bearer_auth_service(&repos),
        ));
        Self {
            hello_service,
            user_service,
            validate_bearer_auth_middleware_service,
        }
    }
}
