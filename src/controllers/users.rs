use crate::application::controllers::user_handle::{
    ban_user_handle, create_user_handle, delate_user_handle, disable_mfa_user_handle,
    get_user_count_handle, get_user_handle, get_users_handle, unban_user_handle,
    update_user_handle,
};
use crate::setup::Container;
use actix_web::web::{self, ServiceConfig};
use std::sync::Arc;

pub fn configure(cfg: &mut ServiceConfig, container: Arc<Container>) {
    let middleware = container.validate_bearer_auth_middleware_service.clone();

    cfg.service(
        web::scope("/users")
            .app_data(web::Data::from(container.create_user_service.clone()))
            .app_data(web::Data::from(container.get_users_service.clone()))
            .app_data(web::Data::from(container.get_user_service.clone()))
            .app_data(web::Data::from(container.update_user_service.clone()))
            .app_data(web::Data::from(container.del_user_service.clone()))
            .app_data(web::Data::from(container.get_user_count_service.clone()))
            .app_data(web::Data::from(container.ban_user_count_service.clone()))
            .app_data(web::Data::from(container.unban_user_count_service.clone()))
            .app_data(web::Data::from(container.disable_mfa_user_service.clone()))
            .wrap(middleware)
            .route("", web::post().to(create_user_handle))
            .route("", web::get().to(get_users_handle))
            .route("/count", web::get().to(get_user_count_handle))
            .route("/{user_id}", web::get().to(get_user_handle))
            .route("/{user_id}", web::patch().to(update_user_handle))
            .route("/{user_id}", web::delete().to(delate_user_handle))
            .route("/{user_id}/ban", web::post().to(ban_user_handle))
            .route("/{user_id}/unban", web::post().to(unban_user_handle))
            .route("/{user_id}/mfa", web::delete().to(disable_mfa_user_handle)),
    );
}
