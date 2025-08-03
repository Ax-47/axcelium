use crate::application::repositories::cdc::replicator::ReplicatorRepository;
use async_trait::async_trait;
use scylla_cdc::consumer::{CDCRow, Consumer};
use std::sync::Arc;
pub struct ReplicatorConsumerServiceImpl {
    repo: Arc<dyn ReplicatorRepository>,
}
impl ReplicatorConsumerServiceImpl {
    pub fn new(repo: Arc<dyn ReplicatorRepository>) -> Self {
        Self { repo }
    }
}
#[async_trait]
pub trait ReplicatorConsumerService: Consumer + Send + Sync {}
impl ReplicatorConsumerService for ReplicatorConsumerServiceImpl {}
#[async_trait]
trait Replicator: Send + Sync {
    async fn execute(&mut self, data: CDCRow<'_>) -> anyhow::Result<()>;
}
#[async_trait]
impl Replicator for ReplicatorConsumerServiceImpl {
    async fn execute(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        println!("replicate: {}", data.operation.to_string().as_str());
        match data.operation.to_string().as_str() {
            "RowInsert" => {
                let user = self.repo.parse_user_from_cdcrow(&data)?;
                self.repo.create(user)?;
            }
            "RowUpdate" => {
                let user = self.repo.parse_user_from_cdcrow(&data)?;
                self.repo.update(user)?;
            }
            "RowDelete" => {
                self.repo.delete()?;
            }
            _ => {
                // ignore others or log
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Consumer for ReplicatorConsumerServiceImpl {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        match self.execute(data).await {
            Ok(()) => Ok(()),
            Err(e) => {
                if e.to_string().contains("some specific error") {
                    tracing::warn!("Handled known issue: {}", e);
                    Ok(()) // หรือจะ return Err ก็ได้
                } else {
                    tracing::error!("Unknown error: {:?}", e);
                    Err(e)
                }
            }
        }
    }
}
