use crate::application::repositories::cdc::PrinterConsumerRepository;
use async_trait::async_trait;
use scylla_cdc::consumer::{CDCRow, Consumer};

pub struct PrinterConsumerServiceImpl {
    repo: Box<dyn PrinterConsumerRepository>,
}
impl PrinterConsumerServiceImpl {
    pub fn new(repo: Box<dyn PrinterConsumerRepository>) -> Self {
        Self { repo }
    }
}
#[async_trait]
impl Consumer for PrinterConsumerServiceImpl {
    async fn consume_cdc(&mut self, data: CDCRow<'_>) -> anyhow::Result<()> {
        let mut row_to_print = String::new();
        // Print header with cdc-specific columns independent of the schema of base table
        // (cdc$stream_id, cdc$time, etc.)
        row_to_print.push_str(&self.repo.print_row_change_header(&data));

        // Print columns dependent on the base schema
        // The appearing order of the columns is undefined
        let column_names = data.get_non_cdc_column_names(); // get 
        for column in column_names {
            let value_field_name = column.to_owned();
            let deleted_elems_field_name = column.to_owned() + "_deleted_elements";
            let is_value_deleted_field_name = column.to_owned() + "_deleted";

            if data.column_exists(column) {
                if let Some(value) = data.get_value(column) {
                    row_to_print.push_str(
                        &self.repo.print_field(
                            value_field_name.as_str(),
                            format!("{:?}", value).as_str(),
                        ),
                    );
                } else {
                    row_to_print
                        .push_str(&self.repo.print_field(value_field_name.as_str(), "null"));
                }
            }

            if data.collection_exists(column) {
                row_to_print.push_str(&self.repo.print_field(
                    deleted_elems_field_name.as_str(),
                    format!("{:?}", data.get_deleted_elements(column)).as_str(),
                ));
            }

            if data.column_deletable(column) {
                row_to_print.push_str(&self.repo.print_field(
                    is_value_deleted_field_name.as_str(),
                    format!("{:?}", data.is_value_deleted(column)).as_str(),
                ));
            }
        }

        // Print end line
        row_to_print.push_str(
            "└────────────────────────────────────────────────────────────────────────────┘\n",
        );
        println!("{}", row_to_print);

        Ok(())
    }
}
