use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::App;
use actix_web::Error;

use crate::container::Container;
use crate::routes;
use std::sync::Arc;

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
    App::new().configure(|cfg| routes::configure_routes(cfg, container.clone()))
}
