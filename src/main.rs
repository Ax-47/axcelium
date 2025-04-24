use actix_web::HttpServer;
use axcelium::{container::Container, create_app::create_app, infrastructure::{cache::redis::get_redis_client, database::scylladb::get_db_pool}, init_application};
use std::sync::Arc;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database =Arc::new(get_db_pool().await);
    let container = Arc::new(Container::new(Arc::new(get_redis_client()),database.clone()));
    init_application::InitialCore::new(database.clone()).init_core().await;
    println!("run server");
    let server =
        HttpServer::new(move || create_app(container.clone())).bind(("127.0.0.1", 6969))?;
    server.run().await
}
