use chrono::DateTime;
use scylla_cdc::consumer::CDCRow;

use crate::infrastructure::repositories::cdc::consts::OUTPUT_WIDTH;

pub fn print_row_change_header(data: &CDCRow<'_>) -> String {
    let mut header_to_print = String::new();
    let stream_id = data.stream_id.to_string();
    let (secs, nanos) = data.time.get_timestamp().unwrap().to_unix();
    let timestamp = DateTime::from_timestamp(secs as i64, nanos)
        .unwrap()
        .to_string();
    let operation = data.operation.to_string();
    let batch_seq_no = data.batch_seq_no.to_string();
    let end_of_batch = data.end_of_batch.to_string();
    let time_to_live = data.ttl.map_or("null".to_string(), |ttl| ttl.to_string());

    header_to_print.push_str(
        "┌──────────────────────────── Scylla CDC log row ────────────────────────────┐\n",
    );
    header_to_print.push_str(&print_field("Stream id:", &stream_id));
    header_to_print.push_str(&print_field("Timestamp:", &timestamp));
    header_to_print.push_str(&print_field("Operation type:", &operation));
    header_to_print.push_str(&print_field("Batch seq no:", &batch_seq_no));
    header_to_print.push_str(&print_field("End of batch:", &end_of_batch));
    header_to_print.push_str(&print_field("TTL:", &time_to_live));
    header_to_print.push_str(
        "├────────────────────────────────────────────────────────────────────────────┤\n",
    );
    header_to_print
}

pub fn print_field(field_name: &str, field_value: &str) -> String {
    let mut field_to_print = format!("│ {}: {}", field_name, field_value);
    let left_spaces: i64 =
        OUTPUT_WIDTH - field_name.chars().count() as i64 - field_value.chars().count() as i64;

    for _ in 0..left_spaces {
        field_to_print.push(' ');
    }

    field_to_print.push_str(" │\n");
    field_to_print
}
