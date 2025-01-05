use actix_web::HttpServer;
use axcelium::{container::Container, create_app::create_app};
use std::sync::Arc;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let container = Arc::new(Container::new());
    let server =
        HttpServer::new(move || create_app(container.clone())).bind(("127.0.0.1", 6969))?;
    server.run().await
}
