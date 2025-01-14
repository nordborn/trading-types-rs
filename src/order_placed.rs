use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{my_date_formatter, Liq, Side, Symbol};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderPlaced {
    pub id: String,
    pub symbol: Symbol,
    pub liq: Liq,
    pub side: Side,
    #[serde(with = "my_date_formatter")]
    pub ts: DateTime<Utc>,
}
