use super::super::services::hello_service::HelloService;
use crate::application::dto::response::hello::HelloResponse;
use actix_web::{Result, web};

pub async fn hello_handler(
    hello_service: web::Data<dyn HelloService>,
) -> Result<web::Json<HelloResponse>> {
    let hello = hello_service.hello_world().await;
    Ok(web::Json(HelloResponse { server: hello }))
}
