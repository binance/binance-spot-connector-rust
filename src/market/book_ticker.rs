use crate::http::{request::Request, Method};

/// `GET /api/v3/ticker/bookTicker`
///
/// Best price/qty on the order book for a symbol or symbols.
///
/// * If the symbol is not sent, bookTickers for all symbols will be returned in an array.
///
/// Weight(IP):
/// * `1` for a single symbol;
/// * `2` when the symbol parameter is omitted;
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::book_ticker().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct BookTicker {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
}

impl BookTicker {
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

impl Default for BookTicker {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BookTicker> for Request {
    fn from(request: BookTicker) -> Request {
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
            path: "/api/v3/ticker/bookTicker".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BookTicker;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_book_ticker_convert_to_request_test() {
        let request: Request = BookTicker::new()
            .symbol("BNBUSDT")
            .symbols(vec!["BTCUSDT", "BNBBTC"])
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/ticker/bookTicker".to_owned(),
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
