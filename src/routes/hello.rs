use crate::application::controllers::hello_handle::hello_handler;
use crate::setup::Container;
use actix_web::web::{self, ServiceConfig};
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig, container: Arc<Container>) {
    cfg.service(
        web::scope("/hello")
            .app_data(web::Data::from(container.hello_service.clone()))
            .route("", web::get().to(hello_handler)),
    );
}
