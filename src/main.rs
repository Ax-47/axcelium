use actix_web::HttpServer;
use axcelium::{
    config,
    controllers::{self, cdc::CDCControllerImpl},
    infrastructure::repositories::{
        cache::redis::get_redis_client, database::scylladb::get_db_pool,
        queue::consumer::ConsumerImpl,
    },
    setup,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{signal, task};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::from_file("./config.yaml").unwrap();
    cfg.validate().unwrap();
    // external adapter
    let database = Arc::new(get_db_pool(cfg.database.clone()).await);
    let mut consumer_queue = ConsumerImpl::new(cfg.queue.clone());
    let container = Arc::new(
        setup::Container::new(
            cfg.clone(),
            Arc::new(get_redis_client(cfg.redis)),
            database.clone(),
        )
        .await,
    );
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let consumer_flag = shutdown_flag.clone();
    println!("run server"); // TODO: Seperate to a func
    let server = {
        let container = container.clone();
        HttpServer::new(move || controllers::create_router(container.clone()))
            .bind(("127.0.0.1", 6969))?
    };
    let mut c = CDCControllerImpl::new(database, container.clone()).await;
    task::spawn(async move { c.handle().await });
    task::spawn_blocking(move || {
        consumer_queue.consume_until(consumer_flag);
    });
    signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
    println!("Received Ctrl+C, shutting down...");

    shutdown_flag.store(true, Ordering::SeqCst);
    server.run().await.map_err(|e| {
        eprintln!("Server error: {:?}", e);
        e
    })
}
