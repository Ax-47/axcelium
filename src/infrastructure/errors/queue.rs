use thiserror::Error;

use crate::infrastructure::errors::fulltext_search::FulltextSearchError;

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
