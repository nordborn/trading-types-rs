use std::cmp::{Eq, PartialEq};
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Side {
    #[serde(rename(serialize = "buy", deserialize = "buy"))]
    #[serde(alias = "Buy")]
    Buy,
    #[serde(rename(serialize = "sell", deserialize = "sell"))]
    #[serde(alias = "Sell")]
    Sell,
}

impl fmt::Display for Side {
    // https://stackoverflow.com/questions/32710187/get-enum-as-string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SideError {
    #[error("bad side {0}")]
    BadSide(String),
}

impl Side {
    pub fn is_buy(&self) -> bool {
        matches!(*self, Side::Buy)
    }

    pub fn is_sell(&self) -> bool {
        matches!(*self, Side::Sell)
    }

    pub fn try_from_str(s: &str) -> Result<Self, SideError> {
        match s.to_lowercase().as_str() {
            "buy" => Ok(Side::Buy),
            "bid" => Ok(Side::Buy),
            "sell" => Ok(Side::Sell),
            "ask" => Ok(Side::Sell),
            _ => Err(SideError::BadSide(s.to_string())),
        }
    }

    pub fn invert(&self) -> Side {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

impl TryFrom<&str> for Side {
    type Error = SideError;

    fn try_from(s: &str) -> Result<Self, SideError> {
        Self::try_from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_side_to_json() {
        let mut m: HashMap<&str, Side> = HashMap::new();
        m.insert("side", Side::Buy);
        let v = json!(m);
        assert_eq!(v.to_string(), r#"{"side":"buy"}"#);
    }

    #[test]
    fn test_side_buy_from_json() {
        let m: HashMap<&str, Side> = serde_json::from_str(r#"{"side":"buy"}"#).unwrap();
        if let Some(v) = m.get(&"side") {
            assert!(v.is_buy())
        } else {
            panic!("{:?}", m);
        }
    }

    #[test]
    fn test_side_sell_from_json() {
        let m: HashMap<&str, Side> = serde_json::from_str(r#"{"side":"sell"}"#).unwrap();
        if let Some(v) = m.get(&"side") {
            assert_eq!(v.is_buy(), false)
        } else {
            panic!("{:?}", m);
        }
    }

    #[test]
    fn test_side_sell_from_upper_json() {
        let m: HashMap<&str, Side> = serde_json::from_str(r#"{"side":"Sell"}"#).unwrap();
        if let Some(v) = m.get(&"side") {
            assert!(v.is_sell())
        } else {
            panic!("{:?}", m);
        }
    }

    #[test]
    #[should_panic]
    fn test_side_unknown_from_json() {
        let _m: HashMap<&str, Side> = serde_json::from_str(r#"{"side":"WTF"}"#).unwrap();
    }
}
