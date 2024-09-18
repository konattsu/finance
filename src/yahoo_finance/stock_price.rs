use std::{fmt::Display, ops::Sub};

use serde::{Deserialize, Serialize};

/// 株価, `1..100_000` の制約
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize)]
pub struct StockPrice(f64);

impl StockPrice {
    pub fn new(price: f64) -> Result<Self, String> {
        if !(1_f64..100_000_f64).contains(&price) {
            Err(format!(
                "Failed to new StockPrice; Abnormal prices: {}",
                price
            ))
        } else {
            Ok(Self(price))
        }
    }
}

impl<'de> Deserialize<'de> for StockPrice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let price = f64::deserialize(deserializer)?;
        StockPrice::new(price).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<f64> for StockPrice {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<StockPrice> for f64 {
    fn from(value: StockPrice) -> Self {
        value.0
    }
}

impl Display for StockPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub for StockPrice {
    type Output = f64;
    /// 四捨五入で小数点第2位までに丸める
    fn sub(self, rhs: Self) -> Self::Output {
        ((f64::from(self) - f64::from(rhs)) * 100.0).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_stock_price_gives_valid_1() {
        let price: f64 = 123.456;
        assert_eq!(
            StockPrice::new(price).expect("Failed to parse to `StockPrice`"),
            StockPrice(123.456)
        );
    }

    #[test]
    fn test_for_stock_price_gives_valid_2() {
        let price: f64 = 11111.into();
        assert_eq!(
            StockPrice::new(price).expect("Failed to parse to `StockPrice`"),
            StockPrice(11111.into())
        );
    }

    #[test]
    fn test_for_stock_price_gives_invalid_out_of_range() {
        let invalid_price: f64 = 100_005.into();
        assert_eq!(
            StockPrice::new(invalid_price),
            Err(format!(
                "Failed to new StockPrice; Abnormal prices: {}",
                100_005
            ))
        )
    }
}
