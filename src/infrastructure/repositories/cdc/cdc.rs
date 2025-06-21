use crate::infrastructure::repositories::cdc::{
    consts::{KEYSPACE, ROLES_BY_APP_TABLE, USERS_TABLE},
    consumer_factory::PrinterConsumerFactory,
};
use futures::future::RemoteHandle;
use scylla::client::session::Session;
use scylla_cdc::log_reader::{CDCLogReader, CDCLogReaderBuilder};
use std::{sync::Arc, time::Duration};
pub struct CDCImpl {
    pub users: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
}

use async_trait::async_trait;

impl CDCImpl {
    pub async fn new(session: Arc<Session>) -> Self {
        println!("test");
        let (user_reader, user_handle): (CDCLogReader, RemoteHandle<anyhow::Result<()>>) =
            CDCLogReaderBuilder::new()
                .session(session.clone())
                .keyspace(KEYSPACE)
                .table_name(USERS_TABLE)
                .window_size(Duration::from_secs_f64(60.))
                .safety_interval(Duration::from_secs_f64(30.))
                .sleep_interval(Duration::from_secs_f64(10.))
                .consumer_factory(Arc::new(PrinterConsumerFactory))
                .build()
                .await
                .unwrap();
        let users = (user_reader, Some(user_handle));
        Self { users }
    }
}
pub struct CDCExternalRepositoryImpl {
    pub cdc: CDCImpl,
}
#[async_trait]
pub trait CDCExternalRepository: Sync + Send {
    async fn stop_repo(&mut self);
}

#[async_trait]
impl CDCExternalRepository for CDCExternalRepositoryImpl {
    async fn stop_repo(&mut self) {
        self.cdc.users.0.stop();
        if let Some(handle) = self.cdc.users.1.take() {
            let _ = handle.await;
        }
    }
}
