use async_trait::async_trait;
use scylla_cdc::consumer::{CDCRow, Consumer};
use std::sync::Arc;

use tokio::sync::Mutex;

pub struct ArcConsumerWrapper(pub Arc<Mutex<dyn Consumer + Send + Sync>>);
#[async_trait]
impl Consumer for ArcConsumerWrapper {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        let mut guard = self.0.lock().await;
        guard.consume_cdc(data).await
    }
}
