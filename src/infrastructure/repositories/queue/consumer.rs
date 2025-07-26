use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use kafka::{
    client::GroupOffsetStorage,
    consumer::{Consumer, FetchOffset},
};

use crate::config;

pub struct ConsumerImpl {
    consumer: Consumer,
}
impl ConsumerImpl {
    pub fn new(cfg: config::QueueConfig) -> Self {
        let consumer = Consumer::from_hosts(cfg.urls)
            .with_topic("axcelium".to_owned())
            .with_fallback_offset(FetchOffset::Latest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .with_group("axcelium-consumer-group".to_owned())
            .create()
            .unwrap();

        Self { consumer }
    }
    pub fn consume_until(&mut self, running: Arc<AtomicBool>) {
        println!("running");
        while !running.load(Ordering::SeqCst) {
            for ms in self.consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    println!("{:?}", str::from_utf8(m.value).unwrap());
                }
                self.consumer.consume_messageset(ms).unwrap();
            }
            self.consumer.commit_consumed().unwrap();
        }
        println!("Kafka consumer stopped~!");
    }
}
