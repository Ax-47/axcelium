use crate::application::controllers::hello_handle::hello_handler;
use crate::application::controllers::user_handle::create_user_handle;
use crate::application::middlewares::bearer_auth::ValidateBearerAuth;
use crate::container::Container;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::Error;
use actix_web::{web, App};
use std::sync::Arc;

pub fn create_app(
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
    let hello_service = container.hello_service.clone();
    let user_service = container.user_service.clone();
    let validate_bearer_auth_middleware_service = container.validate_bearer_auth_middleware_service.clone();
    let validate_bearer_auth_middleware = ValidateBearerAuth::new(validate_bearer_auth_middleware_service);
    App::new()
        .app_data(web::Data::from(hello_service.clone()))
        .app_data(web::Data::from(user_service.clone()))
        .wrap(Logger::default())
        .service(web::scope("/hello").route("", web::get().to(hello_handler)))
        .service(web::scope("/users")
            .wrap(validate_bearer_auth_middleware)
            .route("", web::post().to(create_user_handle))
        )
}
