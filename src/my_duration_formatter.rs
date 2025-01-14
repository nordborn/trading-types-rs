use std::time;

/// https://serde.rs/custom-date-format.html
use chrono::Duration;
use serde::{self, Deserialize, Deserializer, Serializer};

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", duration.num_seconds());
    serializer.serialize_str(&s)
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let timestamp = s.parse::<u64>().map_err(serde::de::Error::custom)?;
    Duration::from_std(time::Duration::from_secs(timestamp)).map_err(serde::de::Error::custom)
}
