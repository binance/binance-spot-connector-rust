use crate::http::{request::Request, Method};

/// `GET /api/v3/avgPrice`
///
/// Current average price for a symbol.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::avg_price("BNBUSDT");
/// ```
pub struct AvgPrice {
    symbol: String,
}

impl AvgPrice {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

impl From<AvgPrice> for Request {
    fn from(request: AvgPrice) -> Request {
        let params = vec![("symbol".to_owned(), request.symbol)];

        Request {
            path: "/api/v3/avgPrice".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AvgPrice;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_avg_price_convert_to_request_test() {
        let request: Request = AvgPrice::new("BNBUSDT").into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/avgPrice".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![("symbol".to_owned(), "BNBUSDT".to_string()),],
                sign: false
            }
        );
    }
}
