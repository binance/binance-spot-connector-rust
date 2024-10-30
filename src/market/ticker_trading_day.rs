use crate::http::{request::Request, Method};
use crate::market::rolling_window_price_change_statistics::TickerType;

/// `GET /api/v3/ticker/tradingDay`
///
/// Price change statistics for a trading day.
///
/// * Supported values for timeZone:
///     * Hours and minutes (e.g. -1:00, 05:45)
///     * Only hours (e.g. 0, 8, 4)
///
/// Weight(IP): 4 for each requested `symbol`
///
/// The weight for this request will cap at 200 once the number of symbols in the request is more than 50.
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::ticker_trading_day().symbol("BNBUSDT");
/// ```
pub struct TickerTradingDay {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
    time_zone: Option<String>,
    ticker_type: Option<TickerType>,
}

impl TickerTradingDay {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            time_zone: None,
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

    pub fn time_zone(mut self, time_zone: &str) -> Self {
        self.time_zone = Some(time_zone.to_owned());
        self
    }

    pub fn ticker_type(mut self, ticker_type: TickerType) -> Self {
        self.ticker_type = Some(ticker_type);
        self
    }
}

impl Default for TickerTradingDay {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TickerTradingDay> for Request {
    fn from(request: TickerTradingDay) -> Request {
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

        if let Some(time_zone) = request.time_zone {
            params.push(("timeZone".to_owned(), time_zone));
        }

        if let Some(ticker_type) = request.ticker_type {
            params.push(("type".to_owned(), ticker_type.to_string()));
        }

        Request {
            path: "/api/v3/ticker/tradingDay".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TickerTradingDay;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_ticker_trading_day_convert_to_request_test() {
        let request: Request = TickerTradingDay::new().symbol("BNBUSDT").into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/ticker/tradingDay".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![("symbol".to_owned(), "BNBUSDT".to_string())],
                sign: false
            }
        );
    }
}
