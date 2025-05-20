pub mod hello;
pub mod users;
pub mod refresh_token;

use crate::setup::Container;
use actix_web::App;
use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::ServiceConfig;
use std::sync::Arc;

pub fn configure_routes(cfg: &mut ServiceConfig, container: Arc<Container>) {
    hello::configure(cfg, container.clone());
    users::configure(cfg, container.clone());
    refresh_token::configure(cfg, container.clone());
}

pub fn create_router(
    container: Arc<Container>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new().configure(|cfg| configure_routes(cfg, container.clone()))
}
