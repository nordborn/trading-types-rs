use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Amount, Depth, Liq, Price};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DepthMap {
    pub asks: HashMap<Price, Liq>,
    pub bids: HashMap<Price, Liq>,
}

impl DepthMap {
    pub fn new() -> Self {
        Self {
            asks: Default::default(),
            bids: Default::default(),
        }
    }

    pub fn replace(&mut self, other: Self) {
        *self = other;
    }

    pub fn into_depth(self) -> Depth {
        let mut d = Depth::new();
        d.asks = self.asks.into_values().collect();
        d.bids = self.bids.into_values().collect();
        d.asks.sort_by(|x, y| x.p.partial_cmp(&y.p).unwrap()); // ascending
        d.bids.sort_by(|x, y| y.p.partial_cmp(&x.p).unwrap()); // descending
        d
    }

    pub fn update(&mut self, other: &Self) {
        other.asks.iter().for_each(|(&p, &l)| {
            if l.a == Amount(0.0) {
                self.asks.remove(&p);
            } else {
                self.asks.entry(p).and_modify(|e| *e = l).or_insert(l);
            }
        });
        other.bids.iter().for_each(|(&p, &l)| {
            if l.a == Amount(0.0) {
                self.bids.remove(&p);
            } else {
                self.bids.entry(p).and_modify(|e| *e = l).or_insert(l);
            }
        });
    }
}
