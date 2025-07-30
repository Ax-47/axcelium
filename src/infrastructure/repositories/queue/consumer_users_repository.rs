use futures::stream::{FusedStream, FuturesUnordered};
use kafka::consumer::MessageSets;
use std::{sync::Arc, time::Duration};
use tokio::sync::watch;

use crate::{
    domain::entities::user::User,
    infrastructure::{
        errors::queue::QueueOperationError,
        models::queue::users::QueueUser,
        repositories::{
            fulltext_search::user_fulltext_search::UserFulltextSearchRepository,
            queue::consumer::ConsumerRepository,
        },
    },
};
pub struct UserConsumerRepositoryImpl {
    consumer_repo: Box<dyn ConsumerRepository>,
    fulltext_search_repo: Arc<dyn UserFulltextSearchRepository>,
}

impl UserConsumerRepositoryImpl {
    pub fn new(
        consumer_repo: Box<dyn ConsumerRepository>,
        fulltext_search_repo: Arc<dyn UserFulltextSearchRepository>,
    ) -> Self {
        Self {
            consumer_repo,
            fulltext_search_repo,
        }
    }
    // pub async fn consumer_handle(self) {
    //     let (fut, handle) = async { self.run().await }.remote_handle();
    //     tokio::task::spawn(fut);
    // }
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
    }
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
            ("create", Some(user)) => self.create(user).await?,
            ("create", None) => return Err(QueueOperationError::MissingUser),
            ("update", None) => { /* TODO */ }
            ("delete", None) => { /* TODO */ }
            (op, _) => eprintln!("Unknown operation: {}", op),
        }
        Ok(())
    }
    async fn create(&self, user: User) -> Result<(), QueueOperationError> {
        self.fulltext_search_repo.create(user).await?;
        Ok(())
    }
}
