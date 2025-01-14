use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{my_date_formatter, Liq, Side};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderToPlace {
    pub liq: Liq,
    pub side: Side,
    #[serde(with = "my_date_formatter")]
    pub ts: DateTime<Utc>,
}

impl OrderToPlace {
    pub fn new(liq: Liq, side: Side) -> Self {
        Self {
            liq,
            side,
            ts: Utc::now(),
        }
    }
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{Amount, Price};

    #[test]
    fn test_order_to_place_to_json() {
        let o = OrderToPlace::new(Liq::from_pa(Price(1.0), Amount(2.0)), Side::Sell);
        let actual = json!(o).to_string();
        println!("{:?}", actual);
        let expected_part = r#"{"liq":{"a":2.0,"p":1.0,"w":2.0},"side":"sell"#;
        println!("{:?}", expected_part);
        let has_part = actual.contains(expected_part);
        assert_eq!(has_part, true)
    }
}
