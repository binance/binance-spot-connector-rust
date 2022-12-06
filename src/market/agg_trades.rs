use crate::http::{request::Request, Method};

/// `GET /api/v3/aggTrades`
///
/// Get compressed, aggregate trades. Trades that fill at the time, from the same order, with the same price will have the quantity aggregated.
/// * If `startTime` and `endTime` are sent, time between startTime and endTime must be less than 1 hour.
/// * If `fromId`, `startTime`, and `endTime` are not sent, the most recent aggregate trades will be returned.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::agg_trades("BNBUSDT").from_id(123).start_time(1640995200000).end_time(1640995200000).limit(500);
/// ```
pub struct AggTrades {
    symbol: String,
    from_id: Option<u64>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl AggTrades {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl From<AggTrades> for Request {
    fn from(request: AggTrades) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(from_id) = request.from_id {
            params.push(("fromId".to_owned(), from_id.to_string()));
        }

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/api/v3/aggTrades".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AggTrades;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_agg_trades_convert_to_request_test() {
        let request: Request = AggTrades::new("BNBUSDT")
            .from_id(123)
            .start_time(1640995200000)
            .end_time(1640995200000)
            .limit(500)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/aggTrades".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("fromId".to_owned(), "123".to_string()),
                    ("startTime".to_owned(), "1640995200000".to_string()),
                    ("endTime".to_owned(), "1640995200000".to_string()),
                    ("limit".to_owned(), "500".to_string()),
                ],
                sign: false
            }
        );
    }
}
