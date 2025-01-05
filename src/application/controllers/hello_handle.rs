use crate::{
    application::dto::hello_dto::Hello, infrastructure::services::hello_service::HelloService,
};
use actix_web::{web, Result};

pub async fn hello_handler(hello_service: web::Data<dyn HelloService>) -> Result<web::Json<Hello>> {
    let hello = hello_service.hello_world().await;
    Ok(web::Json(Hello { server: hello }))
}
