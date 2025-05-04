use actix_web::web::{self, ServiceConfig};
use crate::application::controllers::user_handle::create_user_handle;
use crate::application::middlewares::bearer_auth::ValidateBearerAuth;
use crate::setup::Container;
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig, container: Arc<Container>) {
    let middleware = ValidateBearerAuth::new(container.validate_bearer_auth_middleware_service.clone());

    cfg.service(
        web::scope("/users")
            .app_data(web::Data::from(container.user_service.clone()))
            .wrap(middleware)
            .route("", web::post().to(create_user_handle)),
    );
}