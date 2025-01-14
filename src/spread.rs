use serde::{Deserialize, Serialize};

use crate::{Depth, Liq, Price};

/// Spread of depth with convininent calcs.
/// Very suitable for a depth after depth_util::drop_worth
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Spread {
    pub best_ask: Liq,
    pub best_bid: Liq,
    delta_abs: Price,
    delta_rel: f64,
}

impl Spread {
    pub fn new(best_ask: Liq, best_bid: Liq) -> Self {
        let (delta_abs, delta_rel) = Self::deltas(best_ask, best_bid);
        Self {
            best_ask,
            best_bid,
            delta_abs,
            delta_rel,
        }
    }

    pub fn from_depth(depth: &Depth) -> Option<Self> {
        if depth.asks.is_empty() || depth.bids.is_empty() {
            return None;
        }
        let best_ask = depth.asks[0];
        let best_bid = depth.bids[0];
        Some(Self::new(best_ask, best_bid))
    }

    pub fn delta_abs(&self) -> Price {
        self.delta_abs
    }

    pub fn delta_rel(&self) -> f64 {
        self.delta_rel
    }

    // private helpers

    fn deltas(ask: Liq, bid: Liq) -> (Price, f64) {
        let ask_price = ask.price();
        let bid_price = bid.price();
        let delta_abs: Price = ask_price - bid_price;
        let mid = (ask_price + bid_price) / 2.0;
        let delta_rel = delta_abs / mid;
        (delta_abs, delta_rel)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SpreadError {
    #[error("empty depth")]
    EmptyDepth,
}

impl TryFrom<&Depth> for Spread {
    type Error = SpreadError;

    fn try_from(depth: &Depth) -> Result<Self, SpreadError> {
        Spread::from_depth(depth).ok_or(SpreadError::EmptyDepth)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::Amount;

    #[test]
    fn spread_to_json() {
        let s = Spread::new(
            Liq::from_pa(Price(10.0), Amount(1.0)),
            Liq::from_pa(Price(9.0), Amount(1.0)),
        );
        let v = json!(s);
        let expected = r#"{"best_ask":{"a":1.0,"p":10.0,"w":10.0},"best_bid":{"a":1.0,"p":9.0,"w":9.0},"delta_abs":1.0,"delta_rel":0.10526315789473684}"#;
        assert_eq!(v.to_string().as_str(), expected);
    }
}
