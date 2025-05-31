use crate::config;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;

pub async fn get_db_pool(cfg: config::DatabaseConfig) -> Session {
    let mut builder = SessionBuilder::new();
    for url in cfg.urls {
        builder = builder.known_node(url);
    }
    let session = builder
        .user(cfg.username, cfg.password)
        .build()
        .await
        .unwrap();
    session
}
