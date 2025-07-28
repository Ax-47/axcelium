use crate::config;
use kafka::producer::{Producer, Record};

pub struct ProducerRepositoryImpl {
    producer: Producer,
    topic: String,
}

impl ProducerRepositoryImpl {
    pub fn new(cfg: config::QueueConfig, topic: &str) -> Self {
        let producer = Producer::from_hosts(cfg.urls).create().unwrap();
        Self {
            producer,
            topic: topic.to_string(),
        }
    }
    pub fn send_data_to_topic(&mut self, data: String) {
        let record = Record::from_value(&self.topic, data.as_bytes());
        self.producer.send(&record).unwrap();
    }
}
