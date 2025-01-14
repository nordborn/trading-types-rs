use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::my_date_formatter;
use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub symbol: Symbol,
    pub liq: Liq,
    pub side: Side,
    #[serde(with = "my_date_formatter")]
    pub ts: DateTime<Utc>,
}
