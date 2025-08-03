use anyhow::Result;
use scylla_cdc::consumer::CDCRow;
use uuid::Uuid;

use crate::application::repositories::errors::replicator::ReplicatorRepoError; // หรือจะใช้ std::result::Result<T, ReplicatorRepoError> ก็ได้

pub(crate) fn get_value_owned(
    data: &CDCRow<'_>,
    col: &'static str,
) -> Result<scylla::value::CqlValue, ReplicatorRepoError> {
    data.get_value(col)
        .into_iter()
        .cloned()
        .next()
        .ok_or(ReplicatorRepoError::MissingColumn(col))
}

pub(crate) fn get_uuid(data: &CDCRow<'_>, col: &'static str) -> Result<Uuid, ReplicatorRepoError> {
    get_value_owned(data, col)?
        .as_uuid()
        .ok_or(ReplicatorRepoError::InvalidColumnType(col))
}

pub(crate) fn get_string(
    data: &CDCRow<'_>,
    col: &'static str,
) -> Result<String, ReplicatorRepoError> {
    get_value_owned(data, col)?
        .as_text()
        .map(|s| s.to_string())
        .ok_or(ReplicatorRepoError::InvalidColumnType(col))
}

pub(crate) fn get_i64(data: &CDCRow<'_>, col: &'static str) -> Result<i64, ReplicatorRepoError> {
    get_value_owned(data, col)?
        .as_bigint()
        .map(|v| v as i64)
        .ok_or(ReplicatorRepoError::InvalidColumnType(col))
}

pub(crate) fn get_bool(data: &CDCRow<'_>, col: &'static str) -> Result<bool, ReplicatorRepoError> {
    get_value_owned(data, col)?
        .as_boolean()
        .ok_or(ReplicatorRepoError::InvalidColumnType(col))
}
pub(crate) fn get_optional_string(
    data: &CDCRow<'_>,
    col: &'static str,
) -> Result<Option<String>, ReplicatorRepoError> {
    Ok(data
        .get_value(col)
        .into_iter() // แปลง &Option<CqlValue> เป็น iterator
        .cloned() // clone ค่าใน iterator (owned CqlValue)
        .next() // เอา Option<CqlValue>
        .and_then(|val| val.as_text().map(|s| s.to_string())))
}

pub(crate) fn get_optional_i64(
    data: &CDCRow<'_>,
    col: &'static str,
) -> Result<Option<i64>, ReplicatorRepoError> {
    let val_opt = data.get_value(col).into_iter().cloned().next();
    Ok(val_opt.and_then(|val| val.as_bigint()))
}
