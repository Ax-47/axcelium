use actix_web::HttpServer;
use axcelium::{
    infrastructure::{cache::redis::get_redis_cluster_client, database::scylladb::get_db_pool},
    routes, setup,
};
use std::sync::Arc;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let container = Arc::new(
        setup::Container::new(
            Arc::new(get_redis_cluster_client()),
            Arc::new(get_db_pool().await),
        )
        .await,
    );
    println!("run server");
    let server = HttpServer::new(move || routes::create_router(container.clone()))
        .bind(("127.0.0.1", 6969))?;
    server.run().await
}
