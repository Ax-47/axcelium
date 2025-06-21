use std::sync::Arc;

use async_trait::async_trait;
use scylla_cdc::consumer::{Consumer, ConsumerFactory};
use tokio::sync::Mutex;

use crate::application::services::cdc::{
    consumer_wrapper::ArcConsumerWrapper, printer::PrinterConsumerService,
};

pub struct PrinterConsumerFactory {
    printer_consumer_service: Arc<Mutex<dyn PrinterConsumerService>>,
}

impl PrinterConsumerFactory {
    pub fn new(printer_consumer_service: Arc<Mutex<dyn PrinterConsumerService>>) -> Self {
        Self {
            printer_consumer_service,
        }
    }
}
#[async_trait]
impl ConsumerFactory for PrinterConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(ArcConsumerWrapper(self.printer_consumer_service.clone()))
    }
}
