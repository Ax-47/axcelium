use kafka::consumer::MessageSets;
use std::{
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};
use tokio::sync::watch;

use crate::{
    config::QueueConfig,
    infrastructure::{
        errors::queue::QueueOperationError,
        models::queue::users::QueueUser,
        repositories::queue::{
            consumer::{ConsumerRepository, ConsumerRepositoryImpl},
            topics::USER_TOPIC,
        },
    },
};
pub struct UserConsumerController {
    consumer_repo: Box<dyn ConsumerRepository>,
}
impl UserConsumerController {
    pub fn new(cfg: QueueConfig, running: Arc<AtomicBool>) -> Self {
        let consumer_repo =
            Box::new(ConsumerRepositoryImpl::new(cfg, running, USER_TOPIC.to_owned()).unwrap());
        Self { consumer_repo }
    }
    pub async fn run(&mut self, mut shutdown_rx: watch::Receiver<bool>) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                _ = shutdown_rx.changed() => {
                    println!("🔴 Shutdown signal received in run()");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                        self.consume().await
                }
            }
        }
        Ok(())
    } // should be controller
    async fn consume(&mut self) {
        println!("consuming...");
        match self.consumer_repo.consume_events() {
            Ok(messagesets) => self.handle_messages(messagesets).await,
            Err(e) => {
                eprintln!("consume_events error: {:?}", e);
            }
        }
    }
    async fn handle_messages(&mut self, messagesets: MessageSets) {
        for ms in messagesets.iter() {
            for m in ms.messages() {
                match std::str::from_utf8(m.value) {
                    Ok(v) => {
                        if let Err(e) = self.operate(v).await {
                            eprintln!("operation error: {:?}", e);
                        }
                    }
                    Err(e) => eprintln!("UTF-8 error: {:?}", e),
                }
            }

            if let Err(e) = self.consumer_repo.consume_messageset(ms) {
                eprintln!("consume_messageset error: {:?}", e);
            }
        }

        if let Err(e) = self.consumer_repo.commit_consumed() {
            eprintln!("commit_consumed error: {:?}", e);
        }
    }
    async fn operate(&self, text: &str) -> Result<(), QueueOperationError> {
        let value = serde_json::from_str::<QueueUser>(text)?;
        match (value.operation.as_str(), value.user) {
            ("create", Some(_user)) => { /* TODO */ }
            ("create", None) => return Err(QueueOperationError::MissingUser),
            ("update", None) => { /* TODO */ }
            ("delete", None) => { /* TODO */ }
            (op, _) => eprintln!("Unknown operation: {}", op),
        }
        Ok(())
    }
}
