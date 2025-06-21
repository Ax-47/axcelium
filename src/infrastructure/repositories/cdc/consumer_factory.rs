use async_trait::async_trait;
use scylla_cdc::consumer::{Consumer, ConsumerFactory};

use crate::infrastructure::repositories::cdc::consumer::PrinterConsumer;

pub struct PrinterConsumerFactory;

#[async_trait]
impl ConsumerFactory for PrinterConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(PrinterConsumer)
    }
}
