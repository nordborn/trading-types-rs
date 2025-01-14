use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Depth, DepthMap};

/// Special structure suitable to be a mutex-protected storage for depths of different sources (pairs)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Depths {
    pub vecs: HashMap<String, Depth>, // key can be any, usually str(symbol)
    pub maps: HashMap<String, DepthMap>, // key can be any, usually str(symbol)
}

impl Depths {
    pub fn new() -> Depths {
        Depths {
            vecs: Default::default(),
            maps: Default::default(),
        }
    }

    pub fn upsert(&mut self, key: &str, depth: Depth) {
        let depth_map = depth.clone().into_depthmap();
        self.maps.entry(key.into()).or_default().replace(depth_map);
        self.vecs.entry(key.into()).or_default().replace(depth);
    }

    pub fn update(&mut self, key: &str, depth_change: Depth) {
        let depth_map = depth_change.clone().into_depthmap();
        self.maps
            .entry(key.into())
            .and_modify(|x| x.update(&depth_map))
            .or_insert(depth_map);
        self.vecs
            .entry(key.into())
            .and_modify(|x| *x = self.maps.get(key).unwrap().clone().into_depth())
            .or_insert(depth_change);
    }
}
