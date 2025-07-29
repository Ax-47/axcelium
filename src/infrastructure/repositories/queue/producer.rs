use crate::{config, infrastructure::errors::queue::ProducerError};
use kafka::producer::{Producer, Record};

pub struct ProducerRepositoryImpl {
    producer: Producer,
    topic: Option<String>,
}

impl ProducerRepositoryImpl {
    pub fn new(cfg: config::QueueConfig) -> Self {
        let producer = Producer::from_hosts(cfg.urls)
            .create()
            .expect("Failed to create Kafka producer");
        Self {
            producer,
            topic: None,
        }
    }
}

pub trait ProducerRepository: Send + Sync {
    fn send_data_to_topic(&mut self, data: String) -> Result<(), ProducerError>;
    fn set_topic(&mut self, topic: &str);
}

impl ProducerRepository for ProducerRepositoryImpl {
    fn send_data_to_topic(&mut self, data: String) -> Result<(), ProducerError> {
        let topic = self.topic.as_ref().ok_or(ProducerError::MissingTopic)?;
        let record = Record::from_value(topic, data.as_bytes());
        self.producer.send(&record)?;
        Ok(())
    }
    fn set_topic(&mut self, topic: &str) {
        self.topic = Some(topic.to_string());
    }
}
