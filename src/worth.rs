use core::ops;
use std::cmp::{Eq, PartialEq, PartialOrd};
// use std::cmp::{Ord, Ordering};

use serde::{Deserialize, Serialize};

use crate::{Amount, Price};

/// Worth (cost) = price * amount
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Worth(pub f64);

impl Eq for Worth {}

impl From<(Price, Amount)> for Worth {
    fn from((p, a): (Price, Amount)) -> Self {
        Self::from_pa(p, a)
    }
}

impl Worth {
    pub fn from_pa(p: Price, a: Amount) -> Self {
        Worth(p.0 * a.0)
    }
}

impl ops::Sub<Worth> for Worth {
    type Output = Self;
    fn sub(self, rhs: Worth) -> Self {
        Worth(self.0 - rhs.0)
    }
}

impl ops::Add<Worth> for Worth {
    type Output = Self;
    fn add(self, rhs: Worth) -> Self {
        Worth(self.0 + rhs.0)
    }
}

impl ops::AddAssign<Worth> for Worth {
    fn add_assign(&mut self, rhs: Worth) {
        self.0 += rhs.0;
    }
}

impl ops::SubAssign<Worth> for Worth {
    fn sub_assign(&mut self, rhs: Worth) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f64> for Worth {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Worth(self.0 * rhs)
    }
}

impl ops::MulAssign<f64> for Worth {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 = self.0 * rhs;
    }
}

impl ops::Div<f64> for Worth {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Worth(self.0 / rhs)
    }
}

impl ops::Div<Worth> for f64 {
    type Output = Worth;
    fn div(self, rhs: Worth) -> Worth {
        Worth(self / rhs.0)
    }
}

impl ops::Div<Worth> for Worth {
    type Output = f64;
    fn div(self, rhs: Worth) -> f64 {
        self.0 / rhs.0
    }
}

// for min, max
// https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/
// impl Ord for Worth {
//     fn cmp(&self, rhs: &Self) -> Ordering {
//         // handle infinity
//         self.0.partial_cmp(&rhs.0).unwrap_or(Ordering::Equal)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worth() {
        assert_eq!(Worth(0.5).0, 0.5);
    }

    #[test]
    fn test_json_to_worth() {
        let w: Worth = serde_json::from_str(r#"1.0"#).unwrap();
        assert_eq!(w, Worth(1.0));
    }
}
