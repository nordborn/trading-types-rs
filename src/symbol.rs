use serde::{Deserialize, Serialize};

use crate::{Currency, Exchange};

/// Symbol represents usual pair with extra exchange info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol {
    pub eg: Exchange,
    pub bs: Currency, // base currency, always uppercased
    pub qt: Currency, // quote currency, always uppercased
}

impl Symbol {
    pub fn new(eg: Exchange, cb: Currency, cq: Currency) -> Self {
        Self {
            eg,
            bs: cb.to_uppercase(),
            qt: cq.to_uppercase(),
        }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}/{}", self.eg, self.bs, self.qt)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SymbolError {
    #[error("bad symbol {0}")]
    BadSymbol(String),
}

impl TryFrom<(Exchange, &str)> for Symbol {
    type Error = SymbolError;

    fn try_from((eg, text): (Exchange, &str)) -> Result<Self, Self::Error> {
        let parts1 = text.split('/').collect::<Vec<&str>>();
        if parts1.len() != 2 {
            return Err(SymbolError::BadSymbol(text.to_string()));
        }
        let cb = parts1[0];
        let cq = parts1[1];
        Ok(Self::new(eg, cb.to_string(), cq.to_string()))
    }
}

impl TryFrom<&str> for Symbol {
    type Error = SymbolError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let parts0 = text.split(':').collect::<Vec<&str>>();
        if parts0.len() != 2 {
            return Err(SymbolError::BadSymbol(text.to_string()));
        }
        Self::try_from((parts0[0].to_string(), parts0[1]))
    }
}

impl TryFrom<String> for Symbol {
    type Error = SymbolError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        Self::try_from(text.as_str())
    }
}
