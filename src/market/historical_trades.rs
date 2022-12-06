#![allow(clippy::wrong_self_convention)]

use crate::http::{request::Request, Credentials, Method};

/// `GET /api/v3/historicalTrades`
///
/// Get older market trades.
///
/// Weight(IP): 5
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::historical_trades("BNBUSDT")
///     .limit(100);
/// ```
pub struct HistoricalTrades {
    symbol: String,
    limit: Option<u32>,
    from_id: Option<u64>,
    credentials: Option<Credentials>,
}

impl HistoricalTrades {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            limit: None,
            from_id: None,
            credentials: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<HistoricalTrades> for Request {
    fn from(request: HistoricalTrades) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol)];

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(from_id) = request.from_id {
            params.push(("fromId".to_owned(), from_id.to_string()));
        }

        Request {
            path: "/api/v3/historicalTrades".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HistoricalTrades;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn market_historical_trades_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = HistoricalTrades::new("BNBUSDT")
            .from_id(123)
            .limit(100)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/historicalTrades".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("limit".to_owned(), 100.to_string()),
                    ("fromId".to_owned(), 123.to_string()),
                ],
                sign: false
            }
        );
    }
}
