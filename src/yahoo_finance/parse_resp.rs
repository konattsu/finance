use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use super::{error::YahooError, StockPrice, YahooFinanceResp};

pub struct YahooFinanceInfo {
    /// 通貨
    currency: String,
    /// 最終更新の時刻, utc
    updated_date: DateTime<Utc>,
    /// 最終更新の時刻の時の価格
    present: StockPrice,
    /// 高値
    high_that_day: StockPrice,
    /// 安値
    low_that_day: StockPrice,
    /// 前営業日の終値
    previous_closed: StockPrice,
    /// 銘柄
    name: String,
}

#[allow(unused)]
trait ReturnDueToRespErr {
    fn ret_parse_err(self) -> Result<Self::Item, YahooError>
    where
        Self: Sized;
    type Item;
}

impl<T> ReturnDueToRespErr for Option<T> {
    type Item = T;

    fn ret_parse_err(self) -> Result<Self::Item, YahooError>
    where
        Self: Sized,
    {
        match self {
            Some(v) => Ok(v),
            None => Err(YahooError::ParseError(
                "Failed to access parsed data".into(),
            )),
        }
    }
}

impl<T, E> ReturnDueToRespErr for Result<T, E>
where
    E: ToString,
{
    type Item = T;
    fn ret_parse_err(self) -> Result<Self::Item, YahooError>
    where
        Self: Sized,
    {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(YahooError::ParseError(e.to_string())),
        }
    }
}

#[allow(unused)]
trait ReturnDueToRespEspeciallyToStockPrice {
    fn ret_parse_err_sp(self) -> Result<StockPrice, YahooError>;
}

impl ReturnDueToRespEspeciallyToStockPrice for Option<f64> {
    fn ret_parse_err_sp(self) -> Result<StockPrice, YahooError> {
        StockPrice::new(self.ret_parse_err()?).ret_parse_err()
    }
}

impl YahooFinanceInfo {
    pub fn new(resp: &YahooFinanceResp) -> Result<Self, YahooError> {
        let meta = resp.inner()["chart"]["result"][0]["meta"].clone();

        let currency = meta["currency"].as_str().ret_parse_err()?.to_string();
        let present = meta["regularMarketPrice"].as_f64().ret_parse_err_sp()?;
        let high_that_day = meta["regularMarketDayHigh"].as_f64().ret_parse_err_sp()?;
        let low_that_day = meta["regularMarketDayLow"].as_f64().ret_parse_err_sp()?;
        let previous_closed = meta["chartPreviousClose"].as_f64().ret_parse_err_sp()?;

        let updated_date = meta["regularMarketTime"].as_u64().ret_parse_err()?;
        let updated_date =
            DateTime::from_timestamp(updated_date as i64, 0).ret_parse_err()?;

        let name = meta["longName"].as_str().ret_parse_err()?.to_string();

        Ok(Self {
            currency,
            updated_date,
            present,
            high_that_day,
            low_that_day,
            previous_closed,
            name,
        })
    }

    pub fn inner(&self) -> &Self {
        self
    }

    /// 営業終了してから呼び出してほしい
    pub fn get_result_that_day_ja(&self) -> String {
        let today = self.present - self.previous_closed;

        format!(
            r#"銘柄: {}
通貨: {}
最終更新: {}
現在価格: {}
日次変動幅: {} - {}
前営業の終値: {}
今日: {}
"#,
            self.name,
            self.currency,
            self.updated_date.with_timezone(&Tz::Asia__Tokyo),
            self.present,
            self.low_that_day,
            self.high_that_day,
            self.previous_closed,
            Self::to_string_gives_sign(today)
        )
    }

    pub fn get_present_ja(&self) -> String {
        let today = self.present - self.previous_closed;

        format!(
            r#"銘柄: {}
通貨: {}
最終更新: {}
現在価格: {}
今日: {}
"#,
            self.name,
            self.currency,
            self.updated_date.with_timezone(&Tz::Asia__Tokyo),
            self.present,
            Self::to_string_gives_sign(today)
        )
    }

    fn to_string_gives_sign(f: f64) -> String {
        if f > 0.0 {
            format!("+{}", f)
        } else if f == 0.0 {
            "±0".into()
        } else {
            f.to_string()
        }
    }
}
