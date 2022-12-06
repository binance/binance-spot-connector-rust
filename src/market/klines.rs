use crate::http::{request::Request, Method};
use strum::Display;

#[derive(Copy, Clone, Display)]
pub enum KlineInterval {
    #[strum(serialize = "1m")]
    Minutes1,
    #[strum(serialize = "3m")]
    Minutes3,
    #[strum(serialize = "5m")]
    Minutes5,
    #[strum(serialize = "15m")]
    Minutes15,
    #[strum(serialize = "30m")]
    Minutes30,
    #[strum(serialize = "1h")]
    Hours1,
    #[strum(serialize = "2h")]
    Hours2,
    #[strum(serialize = "4h")]
    Hours4,
    #[strum(serialize = "6h")]
    Hours6,
    #[strum(serialize = "8h")]
    Hours8,
    #[strum(serialize = "12h")]
    Hours12,
    #[strum(serialize = "1d")]
    Days1,
    #[strum(serialize = "3d")]
    Days3,
    #[strum(serialize = "1w")]
    Weeks1,
    #[strum(serialize = "1M")]
    Months1,
}

/// `GET /api/v3/klines`
///
/// Kline/candlestick bars for a symbol.
/// Klines are uniquely identified by their open time.
///
/// * If `startTime` and `endTime` are not sent, the most recent klines are returned.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market::{self, klines::KlineInterval};
///
/// let request = market::klines("BTCUSDT", KlineInterval::Minutes1)
///     .start_time(1654079109000)
///     .end_time(1654079209000);
/// ```
pub struct Klines {
    symbol: String,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl Klines {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_owned(),
            interval,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl From<Klines> for Request {
    fn from(request: Klines) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol),
            ("interval".to_owned(), request.interval.to_string()),
        ];

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/api/v3/klines".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KlineInterval, Klines};
    use crate::http::{request::Request, Method};

    #[test]
    fn market_kline_candlestick_data_convert_to_request_test() {
        let request: Request = Klines::new("BTCUSDT", KlineInterval::Minutes1)
            .start_time(1654079109000)
            .end_time(1654079209000)
            .limit(100)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/klines".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BTCUSDT".to_string()),
                    ("interval".to_owned(), "1m".to_string()),
                    ("startTime".to_owned(), "1654079109000".to_string()),
                    ("endTime".to_owned(), "1654079209000".to_string()),
                    ("limit".to_owned(), "100".to_string())
                ],
                sign: false
            }
        )
    }
}
