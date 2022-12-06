use crate::http::{request::Request, Method};

/// `GET /api/v3/ticker/price`
///
/// Latest price for a symbol or symbols.
///
/// * If the symbol is not sent, prices for all symbols will be returned in an array.
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
/// let request = market::ticker_price().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct TickerPrice {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
}

impl TickerPrice {
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

impl From<TickerPrice> for Request {
    fn from(request: TickerPrice) -> Request {
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
            path: "/api/v3/ticker/price".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for TickerPrice {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::TickerPrice;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_ticker_price_convert_to_request_test() {
        let request: Request = TickerPrice::new()
            .symbol("BNBUSDT")
            .symbols(vec!["BTCUSDT", "BNBBTC"])
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/ticker/price".to_owned(),
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
