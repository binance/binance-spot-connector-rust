use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/margin/borrow-repay`
///
/// Margin account borrow/repay(MARGIN)
///
/// Weight(UID): 1500
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_account_borrow_repay("BNB", "FALSE", "BNBUSDT", "1.0", "BORROW");
/// ```
pub struct MarginAccountBorrowRepay {
    asset: String,
    is_isolated: String,
    symbol: String,
    amount: String,
    type_: String,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginAccountBorrowRepay {
    pub fn new(asset: &str, is_isolated: &str, symbol: &str, amount: &str, type_: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            is_isolated: is_isolated.to_owned(),
            symbol: symbol.to_owned(),
            amount: amount.to_owned(),
            type_: type_.to_owned(),
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

impl From<MarginAccountBorrowRepay> for Request {
    fn from(request: MarginAccountBorrowRepay) -> Request {
        let mut params = vec![
            ("asset".to_owned(), request.asset.to_string()),
            ("isIsolated".to_owned(), request.is_isolated.to_string()),
            ("symbol".to_owned(), request.symbol.to_string()),
            ("amount".to_owned(), request.amount.to_string()),
            ("type".to_owned(), request.type_.to_string()),
        ];

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/borrow-repay".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginAccountBorrowRepay;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_account_borrow_repay_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request =
            MarginAccountBorrowRepay::new("BNB", "FALSE", "BNBUSDT", "1.0", "BORROW")
                .recv_window(5000)
                .credentials(&credentials)
                .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/borrow-repay".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("asset".to_owned(), "BNB".to_string()),
                    ("isIsolated".to_owned(), "FALSE".to_string()),
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("amount".to_owned(), "1.0".to_string()),
                    ("type".to_owned(), "BORROW".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
