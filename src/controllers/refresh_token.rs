use crate::{application::controllers::refresh_token_handle::{create_refresh_token_handle, get_refresh_token_handle, revoke_refresh_token_handle, rotate_refresh_token_handle}, setup::Container};
use actix_web::web::{self, ServiceConfig};
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig, container: Arc<Container>) {
    let middleware = container.validate_bearer_auth_middleware_service.clone();

    cfg.service(
        web::scope("/tokens")
            .app_data(web::Data::from(container.create_refresh_token_service.clone()))
            .app_data(web::Data::from(container.rotate_refresh_token_service.clone()))
            .app_data(web::Data::from(container.revoke_refresh_token_service.clone()))
            .app_data(web::Data::from(container.get_refresh_tokens_by_user_service.clone()))
            .wrap(middleware)
            .route("/", web::post().to(create_refresh_token_handle))
            .route("/rotate", web::post().to(rotate_refresh_token_handle))
            .route("/{user_id}", web::get().to(get_refresh_token_handle))
            .route("/{token_id}/revoke", web::post().to(revoke_refresh_token_handle))
    );
}
