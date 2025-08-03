use axcelium::{
    config,
    controllers::{cdc::CDCControllerImpl, consumers::QueueConsumerImpl},
    infrastructure::repositories::{
        cache::redis::get_redis_client, database::scylladb::get_db_pool,
        fulltext_search::init_fulltext_search,
    },
    setup::{
        self, init_controllers::create_http_server, print::print_server,
        shutdown::spawn_signal_handler,
    },
};
use std::sync::Arc;
use tokio::sync::watch;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = config::Config::from_file("./config.yaml").unwrap();
    // core setup
    cfg.validate().unwrap();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let shutdown_tx_clone = shutdown_tx.clone();
    spawn_signal_handler(shutdown_tx_clone);
    // external adapter
    let database = Arc::new(get_db_pool(cfg.database.clone()).await);
    let fulltext_search = Arc::new(init_fulltext_search(cfg.fulltext_search.clone()));
    // repo
    let repos = setup::repositories::create_all(
        cfg.clone(),
        database.clone(),
        fulltext_search,
        Arc::new(get_redis_client(cfg.redis.clone())),
    )
    .await;
    let services = Arc::new(setup::Container::new(cfg.clone(), repos).await);
    //controllers
    let container_for_server = services.clone();
    let server_future = create_http_server(container_for_server)?;
    let mut c = CDCControllerImpl::new(database, services.clone()).await;
    // FIX: temp
    let consumer_controller = QueueConsumerImpl::new(cfg.queue, shutdown_rx).unwrap();
    // Wait for consumer to end
    tokio::spawn(async move { c.handle().await });
    tokio::spawn(async move {
        consumer_controller.wait().await;
    });
    print_server();
    server_future.await
}
