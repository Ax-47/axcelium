use async_trait::async_trait;
use scylla_cdc::consumer::{Consumer, ConsumerFactory};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::application::services::cdc::{
    consumer_wrapper::ArcConsumerWrapper, replicator::ReplicatorConsumerService,
};

pub struct ReplicatorConsumerFactory {
    replicator_consumer_service: Arc<Mutex<dyn ReplicatorConsumerService>>,
}

impl ReplicatorConsumerFactory {
    pub fn new(replicator_consumer_service: Arc<Mutex<dyn ReplicatorConsumerService>>) -> Self {
        Self {
            replicator_consumer_service,
        }
    }
}
#[async_trait]
impl ConsumerFactory for ReplicatorConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(ArcConsumerWrapper(self.replicator_consumer_service.clone()))
    }
}
