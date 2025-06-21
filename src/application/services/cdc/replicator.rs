use crate::application::repositories::cdc::PrinterConsumerRepository;
use async_trait::async_trait;
use scylla_cdc::consumer::{CDCRow, Consumer};
use std::{fmt::Write, sync::Arc};
pub struct ReplicatorConsumerServiceImpl {
    repo: Arc<dyn PrinterConsumerRepository>,
}

impl ReplicatorConsumerServiceImpl {
    pub fn new(repo: Arc<dyn PrinterConsumerRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
pub trait ReplicatorConsumerService: Consumer + Send + Sync {}
impl ReplicatorConsumerService for ReplicatorConsumerServiceImpl {}
#[async_trait]
trait Replicator: Send + Sync {
    async fn execute(&mut self, data: CDCRow<'_>) -> anyhow::Result<()>;
}

#[async_trait]
impl Replicator for ReplicatorConsumerServiceImpl {
    async fn execute(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        let mut row_to_print = String::new();
        write!(
            &mut row_to_print,
            "{}",
            self.repo.print_row_change_header(&data)
        )?;

        let column_names = data.get_non_cdc_column_names();
        for column in column_names {
            if data.column_exists(column) {
                match data.get_value(column) {
                    Some(value) => write!(
                        &mut row_to_print,
                        "{}",
                        self.repo
                            .print_field(column, format!("{:?}", value).as_str())
                    )?,
                    None => write!(
                        &mut row_to_print,
                        "{}",
                        self.repo.print_field(column, "null")
                    )?,
                };
            }

            if data.collection_exists(column) {
                write!(
                    &mut row_to_print,
                    "{}",
                    self.repo.print_field(
                        &format!("{}_deleted_elements", column),
                        format!("{:?}", data.get_deleted_elements(column)).as_str()
                    )
                )?;
            }

            if data.column_deletable(column) {
                write!(
                    &mut row_to_print,
                    "{}",
                    self.repo.print_field(
                        &format!("{}_deleted", column),
                        format!("{:?}", data.is_value_deleted(column)).as_str()
                    )
                )?;
            }
        }
        writeln!(
            &mut row_to_print,
            "└────────────────────────────────────────────────────────────────────────────┘"
        )?;

        println!("{}", row_to_print);

        Ok(())
    }
}

#[async_trait]
impl Consumer for ReplicatorConsumerServiceImpl {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        self.execute(data).await
    }
}
