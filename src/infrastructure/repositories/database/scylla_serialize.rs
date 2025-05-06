use scylla::value::CqlTimestamp;
use serde::{Deserialize, Deserializer, Serializer};
pub fn serialize_cql_timestamp<S>(ts: &CqlTimestamp, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(ts.0)
}
pub fn deserialize_cql_timestamp<'de, D>(deserializer: D) -> Result<CqlTimestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = i64::deserialize(deserializer)?;
    Ok(CqlTimestamp(millis))
}
pub fn serialize_optional_cql_timestamp<S>(
    ts: &Option<CqlTimestamp>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match ts {
        Some(t) => serializer.serialize_some(&t.0),
        None => serializer.serialize_none(),
    }
}
pub fn deserialize_optional_cql_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<CqlTimestamp>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<i64>::deserialize(deserializer)?;
    Ok(opt.map(CqlTimestamp))
}
