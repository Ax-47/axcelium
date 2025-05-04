use actix_web::HttpServer;
use axcelium::{
    config, infrastructure::{cache::redis::get_redis_client, database::scylladb::get_db_pool}, routes, setup
};
use std::sync::Arc;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg=config::Config::from_file("./config.yaml").unwrap();
    cfg.validate().unwrap();
    let container = Arc::new(
        setup::Container::new(
            cfg.clone(),
            Arc::new(get_redis_client(cfg.redis)),
            Arc::new(get_db_pool(cfg.database).await),
        )
        .await,
    );
    println!("run server");
    let server = HttpServer::new(move || routes::create_router(container.clone()))
        .bind(("127.0.0.1", 6969))?;
    server.run().await
}
