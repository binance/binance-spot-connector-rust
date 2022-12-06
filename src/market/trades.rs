use crate::http::{request::Request, Method};

/// `GET /api/v3/trades`
///
/// Get recent trades.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::trades("BNBUSDT").limit(500);
/// ```
pub struct Trades {
    symbol: String,
    limit: Option<u32>,
}

impl Trades {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            limit: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl From<Trades> for Request {
    fn from(request: Trades) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/api/v3/trades".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Trades;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_trades_convert_to_request_test() {
        let request: Request = Trades::new("BNBUSDT").limit(500).into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/trades".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("limit".to_owned(), "500".to_string()),
                ],
                sign: false
            }
        );
    }
}
