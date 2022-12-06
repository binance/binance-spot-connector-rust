use crate::http::{request::Request, Method};

/// `GET /api/v3/exchangeInfo`
///
/// Current exchange trading rules and symbol information
///
/// * If any symbol provided in either symbol or symbols do not exist, the endpoint will throw an error.
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::exchange_info().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct ExchangeInfo {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
}

impl ExchangeInfo {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
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
}

impl From<ExchangeInfo> for Request {
    fn from(request: ExchangeInfo) -> Request {
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

        Request {
            path: "/api/v3/exchangeInfo".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for ExchangeInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ExchangeInfo;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_exchange_info_convert_to_request_test() {
        let request: Request = ExchangeInfo::new()
            .symbol("BNBUSDT")
            .symbols(vec!["BTCUSDT", "BNBBTC"])
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/exchangeInfo".to_owned(),
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
