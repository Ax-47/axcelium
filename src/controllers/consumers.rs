use futures::{FutureExt, future::RemoteHandle, lock::Mutex};
use std::sync::Arc;
use tokio::sync::watch;

use crate::{
    application::controllers::queue::consumer::{ConsumerCotroller, ConsumerCotrollerImpl},
    config,
    infrastructure::{
        errors::queue::QueueOperationError, models::queue::users::QueueUser,
        repositories::queue::topics::USER_TOPIC,
    },
};
use kafka::Error as KafkaError;
pub struct QueueConsumerImpl {
    pub user_handle: RemoteHandle<()>,
}

impl QueueConsumerImpl {
    pub fn new(
        cfg: config::QueueConfig,
        shutdown_rx: watch::Receiver<bool>,
    ) -> Result<Self, KafkaError> {
        let user_consumer_controller = Arc::new(Mutex::new(ConsumerCotrollerImpl::new(
            cfg,
            shutdown_rx.clone(),
            USER_TOPIC.to_string(),
        )?))
            as Arc<Mutex<dyn ConsumerCotroller<QueueUser, QueueOperationError>>>;

        let (fut, user_handle) = async move {
            let mut locked = user_consumer_controller.lock().await;
            let _ = locked.run().await;
        }
        .remote_handle();

        tokio::spawn(fut);
        Ok(Self { user_handle })
    }

    pub async fn wait(self) {
        let _ = self.user_handle.await;
    }
}
