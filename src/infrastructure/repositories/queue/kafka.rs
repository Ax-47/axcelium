use crate::config;
use kafka::{
    consumer::{Consumer, FetchOffset},
    producer::Producer,
};

pub async fn init_consumer(cfg: config::QueueConfig) -> Consumer {
    Consumer::from_hosts(cfg.urls)
        .with_topic("axcelium".to_owned())
        .with_fallback_offset(FetchOffset::Latest)
        .with_group("axcelium-consumer-group".to_owned())
        .create()
        .unwrap()
}

pub async fn init_producer(cfg: config::QueueConfig) -> Producer {
    Producer::from_hosts(cfg.urls).create().unwrap()
}
