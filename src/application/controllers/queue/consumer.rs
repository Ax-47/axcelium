use std::{collections::HashMap, future::Future, pin::Pin, str, sync::Arc, time::Duration};

use async_trait::async_trait;
use kafka::{
    Error as KafkaError,
    client::GroupOffsetStorage,
    consumer::{Consumer, FetchOffset, MessageSet, MessageSets},
};
use tokio::sync::watch;

use crate::{config, infrastructure::models::queue::queue_payload::QueueOperation};

/// Type alias for a boxed async operation handler
pub type OperationFn<T, E> =
    dyn Fn(&T) -> Pin<Box<dyn Future<Output = Result<(), E>> + Send>> + Send + Sync;

/// Consumer interface
#[async_trait]
pub trait ConsumerCotroller<T, E>: Send + Sync
where
    T: serde::de::DeserializeOwned + QueueOperation + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + std::convert::From<serde_json::Error> + 'static,
{
    fn add_operation(&mut self, op_type: &'static str, op: Arc<OperationFn<T, E>>);
    async fn run(&mut self) -> anyhow::Result<()>;
}

/// Kafka consumer implementation
pub struct ConsumerCotrollerImpl<T, E>
where
    T: serde::de::DeserializeOwned + QueueOperation + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    topic: String,
    consumer: Consumer,
    shutdown_rx: watch::Receiver<bool>,
    operation_map: HashMap<&'static str, Arc<OperationFn<T, E>>>,
}

impl<T, E> ConsumerCotrollerImpl<T, E>
where
    T: serde::de::DeserializeOwned + QueueOperation + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + std::convert::From<serde_json::Error> + 'static,
{
    pub fn new(
        cfg: config::QueueConfig,
        shutdown_rx: watch::Receiver<bool>,
        topic: String,
    ) -> Result<Self, KafkaError> {
        let consumer = Consumer::from_hosts(cfg.urls)
            .with_topic(topic.clone())
            .with_fallback_offset(FetchOffset::Latest)
            .with_group("axcelium".to_string())
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()?;

        Ok(Self {
            topic,
            consumer,
            shutdown_rx,
            operation_map: HashMap::new(),
        })
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

    async fn handle_messages(&mut self, messagesets: MessageSets) {
        for ms in messagesets.iter() {
            for m in ms.messages() {
                match str::from_utf8(m.value) {
                    Ok(v) => {
                        println!("{v}");
                        if let Err(e) = self.operate(v).await {
                            eprintln!("operation error: {:?}", e);
                        }
                    }
                    Err(e) => eprintln!("UTF-8 error: {:?}", e),
                }
            }

            if let Err(e) = self.consume_messageset(ms) {
                eprintln!("consume_messageset error: {:?}", e);
            }
        }

        if let Err(e) = self.commit_consumed() {
            eprintln!("commit_consumed error: {:?}", e);
        }
    }

    pub async fn operate(&self, text: &str) -> Result<(), E> {
        let value = serde_json::from_str::<T>(text)?;
        let op = value.operation();

        if let Some(handler) = self.operation_map.get(op) {
            handler(&value).await
        } else {
            eprintln!("Unknown operation: {}", op);
            Ok(())
        }
    }

    async fn consume(&mut self) {
        match self.consume_events() {
            Ok(messagesets) => self.handle_messages(messagesets).await,
            Err(e) => {
                eprintln!("consume_events error: {:?}", e);
            }
        }
    }
}

#[async_trait]
impl<T, E> ConsumerCotroller<T, E> for ConsumerCotrollerImpl<T, E>
where
    T: serde::de::DeserializeOwned + QueueOperation + Send + Sync + 'static,
    E: std::error::Error + Send + Sync + std::convert::From<serde_json::Error> + 'static,
{
    fn add_operation(&mut self, op_type: &'static str, op: Arc<OperationFn<T, E>>) {
        self.operation_map.insert(op_type, op);
    }
    async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                _ = self.shutdown_rx.changed() => {
                    println!("🔴 Shutdown signal received in {}'s run()",self.topic);
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    self.consume().await;
                }
            }
        }
        Ok(())
    }
}
