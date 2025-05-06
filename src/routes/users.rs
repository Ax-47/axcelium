use actix_web::web::{self, ServiceConfig};
use crate::application::controllers::user_handle::{create_user_handle, get_users_handle};
use crate::setup::Container;
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig, container: Arc<Container>) {
    let middleware = container.validate_bearer_auth_middleware_service.clone();

    cfg.service(
        web::scope("/users")
            .app_data(web::Data::from(container.create_user_service.clone()))
            .app_data(web::Data::from(container.get_users_service.clone()))
            .wrap(middleware)
            .route("", web::post().to(create_user_handle))
            .route("", web::get().to(get_users_handle)),
    );
}