use crate::http::{request::Request, Method};

/// `GET /api/v3/ticker`
///
/// The window used to compute statistics is typically slightly wider than requested windowSize.
///
/// openTime for /api/v3/ticker always starts on a minute, while the closeTime is the current time of the request. As such, the effective window might be up to 1 minute wider than requested.
///
/// E.g. If the closeTime is 1641287867099 (January 04, 2022 09:17:47:099 UTC) , and the windowSize is 1d. the openTime will be: 1641201420000 (January 3, 2022, 09:17:00 UTC)
///
/// Weight(IP): 2 for each requested symbol regardless of windowSize.
///
/// The weight for this request will cap at 100 once the number of symbols in the request is more than 50.
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::rolling_window_price_change_statistics().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct RollingWindowPriceChangeStatistics {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
    window_size: Option<String>,
}

impl RollingWindowPriceChangeStatistics {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            window_size: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn symbols(mut self, symbols: Vec<&str>) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn window_size(mut self, window_size: &str) -> Self {
        self.window_size = Some(window_size.to_owned());
        self
    }
}

impl Default for RollingWindowPriceChangeStatistics {
    fn default() -> Self {
        Self::new()
    }
}

impl From<RollingWindowPriceChangeStatistics> for Request {
    fn from(request: RollingWindowPriceChangeStatistics) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(symbols) = request.symbols {
            params.push((
                "symbols".to_owned(),
                format!("[\"{}\"]", symbols.join("\",\"")),
            ));
        }

        if let Some(window_size) = request.window_size {
            params.push(("windowSize".to_owned(), window_size));
        }

        Request {
            path: "/api/v3/ticker".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RollingWindowPriceChangeStatistics;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_rolling_window_price_change_statistics_convert_to_request_test() {
        let request: Request = RollingWindowPriceChangeStatistics::new()
            .symbol("BNBUSDT")
            .symbols(vec!["BTCUSDT", "BNBBTC"])
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/ticker".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("symbols".to_owned(), "[\"BTCUSDT\",\"BNBBTC\"]".to_string()),
                ],
                sign: false
            }
        );
    }
}
