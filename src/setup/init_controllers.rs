use crate::{controllers, setup::Container};
use actix_web::HttpServer;
use std::sync::Arc;

pub fn create_http_server(services: Arc<Container>) -> std::io::Result<actix_web::dev::Server> {
    let container_for_server = services.clone();
    let server = HttpServer::new(move || controllers::create_router(container_for_server.clone()))
        .bind(("127.0.0.1", 6969))?
        .run();
    Ok(server)
}
