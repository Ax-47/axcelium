use crate::application::repositories::cdc::PrinterConsumerRepositoryImpl;
use crate::application::services::cdc::PrinterConsumerServiceImpl;
use crate::application::services::cdc::consumer_wrapper::ArcConsumerWrapper;
use crate::infrastructure::repositories::cdc::consts::{KEYSPACE, ROLES_BY_APP_TABLE, USERS_TABLE};
use async_trait::async_trait;
use futures::future::RemoteHandle;
use scylla::client::session::Session;
use scylla_cdc::consumer::{Consumer, ConsumerFactory};
use scylla_cdc::log_reader::{CDCLogReader, CDCLogReaderBuilder};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
pub struct CDCControllerImpl {
    pub users: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
    pub roles: (CDCLogReader, Option<RemoteHandle<anyhow::Result<()>>>),
}

impl CDCControllerImpl {
    pub async fn new(session: Arc<Session>) -> Self {
        let print_repo = Arc::new(PrinterConsumerRepositoryImpl);
        let print_service = Arc::new(Mutex::new(PrinterConsumerServiceImpl::new(print_repo)));
        let (user_reader, user_handle): (CDCLogReader, RemoteHandle<anyhow::Result<()>>) =
            CDCLogReaderBuilder::new()
                .session(session.clone())
                .keyspace(KEYSPACE)
                .table_name(USERS_TABLE)
                .window_size(Duration::from_secs_f64(60.))
                .safety_interval(Duration::from_secs_f64(30.))
                .sleep_interval(Duration::from_secs_f64(10.))
                .consumer_factory(Arc::new(PrinterConsumerFactory::new(print_service.clone())))
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
                .consumer_factory(Arc::new(PrinterConsumerFactory::new(print_service)))
                .build()
                .await
                .unwrap();
        let roles = (role_reader, Some(role_handle));
        Self { users, roles }
    }
}

struct PrinterConsumerFactory {
    printer_consumer_service: Arc<Mutex<dyn Consumer + Sync>>,
}

impl PrinterConsumerFactory {
    fn new(printer_consumer_service: Arc<Mutex<dyn Consumer + Sync>>) -> Self {
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
