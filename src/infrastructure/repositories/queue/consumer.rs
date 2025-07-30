use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use kafka::{
    Error as KafkaError,
    client::GroupOffsetStorage,
    consumer::{Consumer, FetchOffset, Message, MessageSet, MessageSets},
};

use crate::config;
use serde::de::Error as DeError;
use serde_json::{Error as SerdeError, Value};
use std::str;

pub struct ConsumerRepositoryImpl {
    consumer: Consumer,
    running: Arc<AtomicBool>,
}

pub trait ConsumerRepository: Send + Sync {
    fn is_running(&self) -> bool;
    fn get_event_data(&self, m: &Message) -> Result<Value, SerdeError>;
    fn consume_events(&mut self) -> Result<MessageSets, KafkaError>;
    fn consume_messageset(&mut self, ms: MessageSet) -> Result<(), KafkaError>;
    fn commit_consumed(&mut self) -> Result<(), KafkaError>;
}
impl ConsumerRepositoryImpl {
    pub fn new(
        cfg: config::QueueConfig,
        running: Arc<AtomicBool>,
        topic: String,
    ) -> Result<Self, KafkaError> {
        let consumer = Consumer::from_hosts(cfg.urls)
            .with_topic(topic)
            .with_fallback_offset(FetchOffset::Latest)
            .with_group("axcelium".to_owned())
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()?;

        Ok(Self { consumer, running })
    }
}

impl ConsumerRepository for ConsumerRepositoryImpl {
    fn is_running(&self) -> bool {
        !self.running.load(Ordering::SeqCst)
    }

    fn get_event_data(&self, m: &Message) -> Result<Value, SerdeError> {
        let event = str::from_utf8(m.value).map_err(|e| SerdeError::custom(e.to_string()))?;
        serde_json::from_str(event)
    }

    fn consume_events(&mut self) -> Result<MessageSets, KafkaError> {
        self.consumer.poll()
    }

    fn consume_messageset(&mut self, ms: MessageSet) -> Result<(), KafkaError> {
        self.consumer.consume_messageset(ms)
    }

    fn commit_consumed(&mut self) -> Result<(), KafkaError> {
        self.consumer.commit_consumed()
    }
}
