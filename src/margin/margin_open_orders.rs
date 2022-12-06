use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/openOrders`
///
/// * If the `symbol` is not sent, orders for all symbols will be returned in an array.
/// * When all symbols are returned, the number of requests counted against the rate limiter is equal to the number of symbols currently trading on the exchange
/// * If isIsolated ="TRUE", symbol must be sent.
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_open_orders().symbol("BNBUSDT");
/// ```
pub struct MarginOpenOrders {
    symbol: Option<String>,
    is_isolated: Option<bool>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginOpenOrders {
    pub fn new() -> Self {
        Self {
            symbol: None,
            is_isolated: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn is_isolated(mut self, is_isolated: bool) -> Self {
        self.is_isolated = Some(is_isolated);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl Default for MarginOpenOrders {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MarginOpenOrders> for Request {
    fn from(request: MarginOpenOrders) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(is_isolated) = request.is_isolated {
            params.push((
                "isIsolated".to_owned(),
                is_isolated.to_string().to_uppercase(),
            ));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/openOrders".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginOpenOrders;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_open_orders_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginOpenOrders::new()
            .symbol("BNBUSDT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/openOrders".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
