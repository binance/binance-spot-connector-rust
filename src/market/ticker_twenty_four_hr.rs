use crate::http::{request::Request, Method};
use crate::market::rolling_window_price_change_statistics::TickerType;

/// `GET /api/v3/ticker/24hr`
///
/// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
///
/// * If the symbol is not sent, tickers for all symbols will be returned in an array.
///
/// Weight(IP):
/// * `2` for a single symbol;
/// * `2` for 1-20 `symbols` sent;
/// * `40` for 21-100 `symbols` sent;
/// * `80` for 101 or more `symbols` sent;
/// * `80` when the symbol parameter is omitted;
/// * `80` when the symbols parameter is omitted;
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::ticker_twenty_four_hr().symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct Ticker24hr {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
    ticker_type: Option<TickerType>,
}

impl Ticker24hr {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            ticker_type: None,
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

    pub fn ticker_type(mut self, ticker_type: TickerType) -> Self {
        self.ticker_type = Some(ticker_type);
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

        if let Some(ticker_type) = request.ticker_type {
            params.push(("type".to_owned(), ticker_type.to_string()));
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
        let request: Request = Ticker24hr::new().symbols(vec!["BTCUSDT", "BNBBTC"]).into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/ticker/24hr".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![("symbols".to_owned(), "[\"BTCUSDT\",\"BNBBTC\"]".to_string())],
                sign: false
            }
        );
    }
}
