use crate::{application::controllers::role_handle::create_role_handler, setup::Container};
use actix_web::web::{self, ServiceConfig};
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig, container: Arc<Container>) {
    let middleware = container.validate_bearer_auth_middleware_service.clone();

    cfg.service(
        web::scope("/roles")
            .app_data(web::Data::from(container.create_role_service.clone()))
            .wrap(middleware)
            .route("/", web::post().to(create_role_handler)),
    );
}
