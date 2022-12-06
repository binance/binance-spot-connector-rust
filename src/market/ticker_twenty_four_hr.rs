use crate::http::{request::Request, Method};

/// `GET /api/v3/ticker/24hr`
///
/// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
///
/// * If the symbol is not sent, tickers for all symbols will be returned in an array.
///
/// Weight(IP):
/// * `1` for a single symbol;
/// * `40` when the symbol parameter is omitted;
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::ticker_twenty_four_hr().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct Ticker24hr {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
}

impl Ticker24hr {
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

impl From<Ticker24hr> for Request {
    fn from(request: Ticker24hr) -> Request {
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
            path: "/api/v3/ticker/24hr".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for Ticker24hr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Ticker24hr;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_ticker_twenty_four_hr_convert_to_request_test() {
        let request: Request = Ticker24hr::new()
            .symbol("BNBUSDT")
            .symbols(vec!["BTCUSDT", "BNBBTC"])
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/ticker/24hr".to_owned(),
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
