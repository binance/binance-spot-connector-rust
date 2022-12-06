use crate::http::{request::Request, Credentials, Method};

/// `DELETE /api/v3/openOrders`
///
/// Cancels all active orders on a symbol.
/// This includes OCO orders.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade;
///
/// let request = trade::cancel_open_orders("BNBUSDT");
/// ```
pub struct CancelOpenOrders {
    symbol: String,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl CancelOpenOrders {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            recv_window: None,
            credentials: None,
        }
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

impl From<CancelOpenOrders> for Request {
    fn from(request: CancelOpenOrders) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/openOrders".to_owned(),
            method: Method::Delete,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CancelOpenOrders;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_cancel_open_orders_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = CancelOpenOrders::new("BNBUSDT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/openOrders".to_owned(),
                credentials: Some(credentials),
                method: Method::Delete,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
