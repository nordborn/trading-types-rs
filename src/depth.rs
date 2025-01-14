use serde::{Deserialize, Serialize};

use crate::{DepthMap, Liq};

/// Depth (order book)
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Depth {
    pub asks: Vec<Liq>,
    pub bids: Vec<Liq>,
}

impl Depth {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn replace(&mut self, other: Self) {
        *self = other;
    }

    pub fn into_depthmap(self) -> DepthMap {
        let mut m = DepthMap::new();
        m.asks = self.asks.into_iter().map(|a| (a.p, a)).collect();
        m.bids = self.bids.into_iter().map(|b| (b.p, b)).collect();
        m
    }
}

impl From<(&[&[String]], &[&[String]])> for Depth {
    fn from((aa, bb): (&[&[String]], &[&[String]])) -> Self {
        let aa: Vec<Liq> = aa.iter().map(|&a| a.into()).collect();
        let bb: Vec<Liq> = bb.iter().map(|&b| b.into()).collect();
        Depth { asks: aa, bids: bb }
    }
}

impl From<(Vec<Vec<std::string::String>>, Vec<Vec<std::string::String>>)> for Depth {
    fn from((aa, bb): (Vec<Vec<std::string::String>>, Vec<Vec<std::string::String>>)) -> Self {
        let aa: Vec<Liq> = aa.iter().map(|a| a.into()).collect();
        let bb: Vec<Liq> = bb.iter().map(|b| b.into()).collect();
        Depth { asks: aa, bids: bb }
    }
}

impl From<(&[&[f64]], &[&[f64]])> for Depth {
    fn from((aa, bb): (&[&[f64]], &[&[f64]])) -> Self {
        let aa: Vec<Liq> = aa.iter().map(|&a| a.into()).collect();
        let bb: Vec<Liq> = bb.iter().map(|&b| b.into()).collect();
        Depth { asks: aa, bids: bb }
    }
}
