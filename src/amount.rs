use core::ops;
use std::cmp::{Eq, PartialEq, PartialOrd};
// use std::cmp::{Ord, Ordering};

use serde::{Deserialize, Serialize};

use crate::{Price, Worth};

// Why not Qty as more "loud" maybe? To separate "qty" and "quote".
// So, Amount
/// Amount (quantity) of base currency.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Amount(pub f64);

impl Eq for Amount {}

impl From<(Price, Worth)> for Amount {
    fn from((p, w): (Price, Worth)) -> Self {
        Self::from_pw(p, w)
    }
}

impl Amount {
    pub fn from_pw(p: Price, w: Worth) -> Self {
        Self(w.0 / p.0)
    }
}

impl ops::Sub<Amount> for Amount {
    type Output = Self;
    fn sub(self, rhs: Amount) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::Add<Amount> for Amount {
    type Output = Self;
    fn add(self, rhs: Amount) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign<Amount> for Amount {
    fn add_assign(&mut self, rhs: Amount) {
        self.0 += rhs.0;
    }
}

impl ops::SubAssign<Amount> for Amount {
    fn sub_assign(&mut self, rhs: Amount) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f64> for Amount {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Div<f64> for Amount {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::Div<Amount> for f64 {
    type Output = Amount;
    fn div(self, rhs: Amount) -> Amount {
        Amount(self / rhs.0)
    }
}

impl ops::Div<Amount> for Amount {
    type Output = f64;
    fn div(self, rhs: Amount) -> f64 {
        self.0 / rhs.0
    }
}

// for min, max
// https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/
// impl Ord for Amount {
//     fn cmp(&self, rhs: &Self) -> Ordering {
//         self.0.partial_cmp(&rhs.0).unwrap_or(Ordering::Equal)
//     }
// }

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_amount_to_json() {
        // https://serde.rs/derive.html
        let a = Amount(1.0);
        let v = json!(a);
        assert_eq!(v, 1.0);
    }

    #[test]
    fn test_json_to_amount() {
        let a: Amount = serde_json::from_str(r#"1.0"#).unwrap();
        assert_eq!(a, Amount(1.0));
    }

    #[test]
    fn test_f64_div_amount() {
        let a = Amount(10.);
        let actual = 1. / a;
        assert_eq!(Amount(0.1), actual)
    }
}
