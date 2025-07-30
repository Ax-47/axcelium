use std::sync::Arc;

use futures::{FutureExt, future::RemoteHandle, lock::Mutex};
use tokio::sync::watch;

use crate::infrastructure::repositories::queue::consumer_users_repository::UserConsumerRepositoryImpl;

pub struct QueueConsumerImpl {
    pub user_handle: RemoteHandle<()>,
}

impl QueueConsumerImpl {
    pub fn new(
        user_consumer_service: Arc<Mutex<UserConsumerRepositoryImpl>>,
        shutdown_rx: watch::Receiver<bool>,
    ) -> Self {
        let (fut, user_handle) = async move {
            let mut locked = user_consumer_service.lock().await;
            let _ = locked.run(shutdown_rx).await;
        }
        .remote_handle();

        tokio::spawn(fut);
        Self { user_handle }
    }

    pub async fn wait(self) {
        let _ = self.user_handle.await;
    }
}
