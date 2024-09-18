use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::DetailResponseError;

use super::error::YahooError;

/// fetchした内容が正しい形式であると高い確率で保障されている
///
/// おそらく100%だが、固有の構造体に変換してないので確約はできない
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct YahooFinanceResp(
    // pub now: StockPrice,
    // pub max_today: StockPrice,
    // pub min_today: StockPrice,
    // pub open: StockPrice,
    // pub previous_close: StockPrice,
    Value,
);

impl YahooFinanceResp {
    pub fn fetch(endpoint_url: &str) -> Result<Self, YahooError> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(endpoint_url)
            .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()
            .map_err(|e| YahooError::NetworkError(e.to_string()))?;

        let status = response.status();

        if status.is_success() {
            // jsonに解析。できないとエラーを返す
            let value: Value = response.json().map_err(|_| {
                YahooError::ParseError("Failed to parse to json format".into())
            })?;
            // responseにerrorというkeyが含まれている。これが存在しないとエラー
            let key = value["chart"].get("error").ok_or_else(|| {
                YahooError::ParseError("Failed to accessing to parsed data".into())
            })?;
            // 上記のerrorというkeyがnullでないとき
            if key.is_null() {
                Ok(Self(value))
            } else {
                Err(YahooError::ParseError(
                    "Failed to accessing to parsed data".into(),
                ))
            }
        } else if status.is_client_error() {
            Err(YahooError::InvalidCall(
                response.text().detail_resp_err("Client"),
            ))
        } else {
            Err(YahooError::ExternalServiceError(
                response.text().detail_resp_err("Server"),
            ))
        }
    }

    pub fn inner(&self) -> &Value {
        &self.0
    }
}
