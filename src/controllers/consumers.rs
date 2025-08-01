use crate::application::controllers::queue::consumer_users::UserConsumerController;
use futures::{FutureExt, future::RemoteHandle, lock::Mutex};
use std::sync::Arc;
use tokio::sync::watch;
pub struct QueueConsumerImpl {
    pub user_handle: RemoteHandle<()>,
}

impl QueueConsumerImpl {
    pub fn new(
        user_consumer_controller: Arc<Mutex<UserConsumerController>>,
        shutdown_rx: watch::Receiver<bool>,
    ) -> Self {
        let (fut, user_handle) = async move {
            let mut locked = user_consumer_controller.lock().await;
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
