use crate::application::repositories::cdc::PrinterConsumerRepositoryImpl;
use crate::application::services::cdc::PrinterConsumerServiceImpl;
use crate::infrastructure::repositories::cdc::consts::{KEYSPACE, ROLES_BY_APP_TABLE, USERS_TABLE};
use async_trait::async_trait;
use futures::future::RemoteHandle;
use scylla::client::session::Session;
use scylla_cdc::consumer::{Consumer, ConsumerFactory};
use scylla_cdc::log_reader::{CDCLogReader, CDCLogReaderBuilder};
use std::{sync::Arc, time::Duration};
pub struct CDCImpl {
    pub users: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
    pub roles: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
}

impl CDCImpl {
    pub async fn new(session: Arc<Session>) -> Self {
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
        let (role_reader, role_handle): (CDCLogReader, RemoteHandle<anyhow::Result<()>>) =
            CDCLogReaderBuilder::new()
                .session(session.clone())
                .keyspace(KEYSPACE)
                .table_name(ROLES_BY_APP_TABLE)
                .window_size(Duration::from_secs_f64(60.))
                .safety_interval(Duration::from_secs_f64(30.))
                .sleep_interval(Duration::from_secs_f64(10.))
                .consumer_factory(Arc::new(PrinterConsumerFactory))
                .build()
                .await
                .unwrap();
        let roles = (role_reader, Some(role_handle));
        Self { users, roles }
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
        self.cdc.roles.0.stop();
        if let Some(handle) = self.cdc.roles.1.take() {
            let _ = handle.await;
        }
    }
}

pub struct PrinterConsumerFactory;
#[async_trait]
impl ConsumerFactory for PrinterConsumerFactory {
    async fn new_consumer(&self) -> Box<dyn Consumer> {
        Box::new(PrinterConsumerServiceImpl::new(Box::new(
            PrinterConsumerRepositoryImpl,
        )))
    }
}
