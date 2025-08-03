use crate::application::controllers::cdc::printer::PrinterConsumerFactory;
use crate::application::controllers::cdc::replicator::ReplicatorConsumerFactory;
use crate::infrastructure::repositories::cdc::consts::{KEYSPACE, ROLES_BY_APP_TABLE, USERS_TABLE};
use crate::setup::Container;
use futures::future::RemoteHandle;
use scylla::client::session::Session;
use scylla_cdc::log_reader::{CDCLogReader, CDCLogReaderBuilder};
use std::{sync::Arc, time::Duration};
pub struct CDCControllerImpl {
    pub user_printer: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
    pub user_replicator: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
    pub role_printer: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
}

impl CDCControllerImpl {
    pub async fn new(session: Arc<Session>, container: Arc<Container>) -> Self {
        let (user_reader, user_handle): (CDCLogReader, RemoteHandle<anyhow::Result<()>>) =
            CDCLogReaderBuilder::new()
                .session(session.clone())
                .keyspace(KEYSPACE)
                .table_name(USERS_TABLE)
                .window_size(Duration::from_secs_f64(60.))
                .safety_interval(Duration::from_secs_f64(30.))
                .sleep_interval(Duration::from_secs_f64(10.))
                .consumer_factory(Arc::new(PrinterConsumerFactory::new(
                    container.printer_service.clone(),
                )))
                .build()
                .await
                .unwrap();
        let user_printer = (user_reader, Some(user_handle));

        let (user_reader, user_handle): (CDCLogReader, RemoteHandle<anyhow::Result<()>>) =
            CDCLogReaderBuilder::new()
                .session(session.clone())
                .keyspace(KEYSPACE)
                .table_name(USERS_TABLE)
                .window_size(Duration::from_secs_f64(60.))
                .safety_interval(Duration::from_secs_f64(30.))
                .sleep_interval(Duration::from_secs_f64(10.))
                .consumer_factory(Arc::new(ReplicatorConsumerFactory::new(
                    container.replicator_service.clone(),
                )))
                .build()
                .await
                .unwrap();
        let user_replicator = (user_reader, Some(user_handle));
        let (role_reader, role_handle): (CDCLogReader, RemoteHandle<anyhow::Result<()>>) =
            CDCLogReaderBuilder::new()
                .session(session.clone())
                .keyspace(KEYSPACE)
                .table_name(ROLES_BY_APP_TABLE)
                .window_size(Duration::from_secs_f64(60.))
                .safety_interval(Duration::from_secs_f64(30.))
                .sleep_interval(Duration::from_secs_f64(10.))
                .consumer_factory(Arc::new(PrinterConsumerFactory::new(
                    container.printer_service.clone(),
                )))
                .build()
                .await
                .unwrap();
        let role_printer = (role_reader, Some(role_handle));
        Self {
            user_printer,
            role_printer,
            user_replicator,
        }
    }
    pub async fn handle(&mut self) {
        if let Some(handle) = self.user_printer.1.take() {
            let _ = handle.await;
        }

        if let Some(handle) = self.user_replicator.1.take() {
            let _ = handle.await;
        }
        if let Some(handle) = self.role_printer.1.take() {
            let _ = handle.await;
        }
    }
}
