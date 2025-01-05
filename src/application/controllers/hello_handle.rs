use actix_web::{web, Result};

use crate::{
    domain::models::hello_models::HelloJSON, infrastructure::services::hello_service::HelloService,
};

pub async fn hello_handler(
    hello_service: web::Data<dyn HelloService>,
) -> Result<web::Json<HelloJSON>> {
    let hello = hello_service.hello_world().await;
    Ok(web::Json(HelloJSON { server: hello }))
}
