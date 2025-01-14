use super::{Amount, Price, Worth};
use serde::{Deserialize, Serialize};

/// Liq (liquidity) is a complex data type that contains all necessary origin information,
/// suitable for further processing. Thus, depth (order book) is a vec of Liqs
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Liq {
    pub(super) p: Price,
    pub(super) a: Amount,
    pub(super) w: Worth,
}

impl Liq {
    pub fn from_pa(p: Price, a: Amount) -> Self {
        Self {
            p,
            a,
            w: Worth::from_pa(p, a),
        }
    }

    pub fn from_pw(p: Price, w: Worth) -> Self {
        Self {
            p,
            a: Amount::from_pw(p, w),
            w,
        }
    }

    pub fn price(&self) -> Price {
        self.p
    }

    pub fn amount(&self) -> Amount {
        self.a
    }

    pub fn worth(&self) -> Worth {
        self.w
    }
}

impl From<(Price, Amount, Worth)> for Liq {
    fn from((p, a, w): (Price, Amount, Worth)) -> Self {
        Self { p, a, w }
    }
}

impl From<(Price, Amount)> for Liq {
    fn from((p, a): (Price, Amount)) -> Self {
        Self::from_pa(p, a)
    }
}

impl From<(Price, Worth)> for Liq {
    fn from((p, w): (Price, Worth)) -> Self {
        Self::from_pw(p, w)
    }
}

impl From<&[String]> for Liq {
    fn from(pa: &[String]) -> Self {
        // must panic on bad incoming data, because this must be fixed on dev stage
        let p = Price(pa[0].parse().unwrap());
        let a = Amount(pa[1].parse().unwrap());
        Self::from((p, a))
    }
}

impl From<&Vec<String>> for Liq {
    fn from(pa: &Vec<String>) -> Self {
        // must panic on bad incoming data, because this must be fixed on dev stage
        let p = Price(pa[0].parse().unwrap());
        let a = Amount(pa[1].parse().unwrap());
        Self::from((p, a))
    }
}

impl From<&[f64]> for Liq {
    fn from(pa: &[f64]) -> Self {
        let p = Price(pa[0]);
        let a = Amount(pa[1]);
        Self::from((p, a))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_liq() {
        let p = Price(10.0);
        let a = Amount(0.5);
        let liq = Liq::from_pa(p, a);
        assert_eq!(liq.worth(), Worth(5.0));
    }

    #[test]
    fn test_liq2() {
        let p = Price(10.0);
        let a = Amount(0.5);
        let liq = Liq::from((p, a));
        assert_eq!(liq.worth(), Worth(5.0));
    }

    #[test]
    fn test_show_liq() {
        let liq = Liq::from_pa(Price(10.0), Amount(0.5));
        dbg!(liq);
        assert!(true);
    }

    #[test]
    fn test_json_to_liq() {
        let r: Liq = serde_json::from_str(r#"{"p":1,"a":2,"w":2}"#).unwrap();
        assert_eq!(r.worth(), Worth(2.0));
    }

    #[test]
    fn test_liq_to_json() {
        // https://serde.rs/derive.html
        let r = Liq::from_pa(Price(1.0), Amount(10.0));
        let v = json!(r);
        assert_eq!(v.to_string(), r#"{"a":10.0,"p":1.0,"w":10.0}"#);
    }
}
