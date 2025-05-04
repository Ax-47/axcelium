pub mod hello;
pub mod users;

use crate::container::Container;
use actix_web::web::ServiceConfig;
use std::sync::Arc;

pub fn configure_routes(cfg: &mut ServiceConfig, container: Arc<Container>) {
    hello::configure(cfg, container.clone());
    users::configure(cfg, container.clone());
}
