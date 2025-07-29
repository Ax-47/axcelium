use actix_web::HttpServer;
use axcelium::{
    config,
    controllers::{self, cdc::CDCControllerImpl},
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
use std::sync::{Arc, atomic::AtomicBool};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let cfg = config::Config::from_file("./config.yaml").unwrap();
    cfg.validate().unwrap();
    // external adapter
    let database = Arc::new(get_db_pool(cfg.database.clone()).await);
    let fulltext_search = Arc::new(init_fulltext_search(cfg.fulltext_search.clone()));

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
    let mut user_consumer_service = Box::new(UserConsumerRepositoryImpl::new(
        u_consumer_repo,
        repos.user_fulltext_search_repo.clone(),
    ));
    let container = Arc::new(setup::Container::new(cfg.clone(), repos).await);
    println!("run server"); // TODO: Seperate to a func
    let container_for_server = container.clone();
    let srv = HttpServer::new(move || controllers::create_router(container_for_server.clone()));
    let srv = srv.bind(("127.0.0.1", 6969))?;
    let server_future = srv.run();
    let mut c = CDCControllerImpl::new(database, container.clone()).await;
    tokio::spawn(async move { c.handle().await }); // TODO: cdc deriver
    // tokio::spawn(async move {
    //     user_consumer_service.consume_until().await;
    // }); // TODO: kafka deriver thread error
    // TODO: close programme
    server_future.await
}
