use actix_web::HttpServer;
use axcelium::{
    config,
    controllers::{self, cdc::CDCControllerImpl, consumers::QueueConsumerImpl},
    infrastructure::repositories::{
        cache::redis::get_redis_client,
        database::scylladb::get_db_pool,
        fulltext_search::init_fulltext_search,
        queue::{
            consumer::ConsumerRepositoryImpl,
            consumer_users_repository::UserConsumerRepositoryImpl,
            producer::ProducerRepositoryImpl,
        },
    },
    setup,
};
use futures::lock::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::{
    signal::unix::{SignalKind, signal},
    sync::watch,
};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let cfg = config::Config::from_file("./config.yaml").unwrap();
    cfg.validate().unwrap();
    // external adapter
    let database = Arc::new(get_db_pool(cfg.database.clone()).await);
    let fulltext_search = Arc::new(init_fulltext_search(cfg.fulltext_search.clone()));
    // derive
    let u_consumer_repo = Box::new(
        ConsumerRepositoryImpl::new(
            cfg.queue.clone(),
            Arc::clone(&shutdown_flag),
            "axcelium-users".to_owned(),
        )
        .unwrap(),
    );

    let repos = setup::repositories::create_all(
        cfg.clone(),
        database.clone(),
        fulltext_search,
        Arc::new(get_redis_client(cfg.redis.clone())),
    )
    .await;
    let user_consumer_service = Arc::new(Mutex::new(UserConsumerRepositoryImpl::new(
        u_consumer_repo,
        repos.user_fulltext_search_repo.clone(),
    )));
    let container = Arc::new(setup::Container::new(cfg.clone(), repos).await);
    println!("run server"); // TODO: Seperate to a func
    let container_for_server = container.clone();
    let srv = HttpServer::new(move || controllers::create_router(container_for_server.clone()));
    let srv = srv.bind(("127.0.0.1", 6969))?;
    let server_future = srv.run();
    let mut c = CDCControllerImpl::new(database, container.clone()).await;
    tokio::spawn(async move { c.handle().await }); // TODO: cdc deriver
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    // Signal handler
    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();
        tokio::select! {
            _ = sigterm.recv() => println!("Received SIGTERM"),
            _ = sigint.recv() => println!("Received SIGINT"),
        }
        let _ = shutdown_tx_clone.send(true); // ส่งสัญญาณ shutdown
    });
    let consumer_controller = QueueConsumerImpl::new(user_consumer_service, shutdown_rx);

    consumer_controller.wait().await;
    // Wait for consumer to end
    // TODO: close programme
    server_future.await
}
