use core::ops;
use std::cmp::{Eq, PartialEq, PartialOrd};
// use std::cmp::{Ord, Ordering};
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

// Price (exchange rate) for base/quote
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Price(pub f64); // NaN, inf are not allowed

impl Hash for Price {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.to_bits().hash(state)
    }
}

impl Eq for Price {}

impl ops::Sub<Price> for Price {
    type Output = Self;
    fn sub(self, rhs: Price) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl ops::Add<Price> for Price {
    type Output = Self;
    fn add(self, rhs: Price) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign<Price> for Price {
    fn add_assign(&mut self, rhs: Price) {
        self.0 += rhs.0;
    }
}

impl ops::SubAssign<Price> for Price {
    fn sub_assign(&mut self, rhs: Price) {
        self.0 -= rhs.0;
    }
}

impl ops::Mul<f64> for Price {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

impl ops::Div<f64> for Price {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self(self.0 / rhs)
    }
}

impl ops::Div<Price> for f64 {
    type Output = Price;
    fn div(self, rhs: Price) -> Price {
        Price(self / rhs.0)
    }
}

impl ops::Div<Price> for Price {
    type Output = f64;
    fn div(self, rhs: Price) -> f64 {
        self.0 / rhs.0
    }
}

// for min, max
// https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/
// impl Ord for Price {
//     fn cmp(&self, rhs: &Self) -> Ordering {
//         self.0.partial_cmp(&rhs.0).unwrap_or(Ordering::Equal)
//     }
// }
