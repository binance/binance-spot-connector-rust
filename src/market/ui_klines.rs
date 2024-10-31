use crate::http::{request::Request, Method};
use crate::market::klines::KlineInterval;

/// `GET /api/v3/uiKlines`
///
/// The request is similar to klines having the same parameters and response.
/// `uiKlines` return modified kline data, optimized for presentation of candlestick charts.
///
/// * If `startTime` and `endTime` are not sent, the most recent Klines are returned.
/// * Supported values for timeZone:
///     * Hours and minutes (e.g. `-1:00`, `05:45`)
///     * Only hours (e.g. `0`, `8`, `4`)
///     * Accepted range is strictly [-12:00 to +14:00] inclusive
/// * If `timeZone` provided, kline intervals are interpreted in that timezone instead of UTC.
/// * Note that `startTime` and `endTime` are always interpreted in UTC, regardless of `timeZone`.
///
/// Weight(IP): 2
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market::{self, klines::KlineInterval};
///
/// let request = market::ui_klines("BTCUSDT", KlineInterval::Minutes1)
///     .start_time(1654079109000)
///     .end_time(1654079209000);
/// ```
pub struct UIKlines {
    symbol: String,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    time_zone: Option<String>,
    limit: Option<u32>,
}

impl UIKlines {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_owned(),
            interval,
            start_time: None,
            end_time: None,
            time_zone: None,
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

    pub fn time_zone(mut self, time_zone: &str) -> Self {
        self.time_zone = Some(time_zone.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl From<UIKlines> for Request {
    fn from(request: UIKlines) -> Request {
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

        if let Some(time_zone) = request.time_zone {
            params.push(("timeZone".to_owned(), time_zone));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/api/v3/uiKlines".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KlineInterval, UIKlines};
    use crate::http::{request::Request, Method};

    #[test]
    fn market_ui_klines_convert_to_request_test() {
        let request: Request = UIKlines::new("BTCUSDT", KlineInterval::Minutes1)
            .start_time(1654079109000)
            .end_time(1654079209000)
            .limit(100)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/uiKlines".to_owned(),
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
