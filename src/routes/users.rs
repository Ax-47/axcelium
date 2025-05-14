use crate::application::controllers::user_handle::{
    create_user_handle, delate_user_handle, get_user_handle, get_users_handle, update_user_handle
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
            .wrap(middleware)
            .route("", web::post().to(create_user_handle))
            .route("", web::get().to(get_users_handle))
            .route("/{user_id}", web::get().to(get_user_handle))
            .route("/{user_id}", web::put().to(update_user_handle)) // todo check prem
            .route("/{user_id}", web::delete().to(delate_user_handle)),// todo check prem
    );
}
