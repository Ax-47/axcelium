use crate::infrastructure::errors::fulltext_search::FulltextSearchError;
use kafka::error::Error as KafkaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueueOperationError {
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    FulltextSearchError(#[from] FulltextSearchError),

    #[error("Missing user for operation")]
    MissingUser,

    #[error("Unknown operation: {0}")]
    UnknownOperation(String),
    // เพิ่ม error อื่นๆ ได้ เช่น KafkaError, DBError เป็นต้น
}

#[derive(Debug, Error)]
pub enum ProducerError {
    #[error("Kafka send error: {0}")]
    Kafka(#[from] KafkaError),

    #[error("Kafka topic is not set")]
    MissingTopic,
}
