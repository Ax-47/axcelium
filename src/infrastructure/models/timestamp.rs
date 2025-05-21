use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

pub fn from_iso8601_to_timestamp<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt: DateTime<Utc> = s
        .parse()
        .map_err(serde::de::Error::custom)?;
    Ok(dt.timestamp())
}