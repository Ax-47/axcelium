use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplicatorRepoError {
    #[error("Missing required column: {0}")]
    MissingColumn(&'static str),

    #[error("Invalid data type in column: {0}")]
    InvalidColumnType(&'static str),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Kafka send error")]
    KafkaSendError,

    #[error("Lock poisoned error")]
    LockError,
}
