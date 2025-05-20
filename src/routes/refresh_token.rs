use crate::{application::controllers::refresh_token_handle::create_refresh_token_handle, setup::Container};
use actix_web::web::{self, ServiceConfig};
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig, container: Arc<Container>) {
    let middleware = container.validate_bearer_auth_middleware_service.clone();

    cfg.service(
        web::scope("/token")
            .app_data(web::Data::from(container.create_user_service.clone()))
            .wrap(middleware)
            .route("/{user_id}", web::post().to(create_refresh_token_handle)),
    );
}
