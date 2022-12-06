use crate::http::{request::Request, Credentials, Method};
use rust_decimal::Decimal;

/// `POST /sapi/v1/margin/loan`
///
/// Apply for a loan.
///
/// * If "isIsolated" = "TRUE", "symbol" must be sent
/// * "isIsolated" = "FALSE" for crossed margin loan
///
/// Weight(UID): 3000
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
/// use rust_decimal_macros::dec;
///
/// let request = margin::margin_borrow("BTC", dec!(1.01)).symbol("BNBUSDT");
/// ```
pub struct MarginBorrow {
    asset: String,
    amount: Decimal,
    is_isolated: Option<bool>,
    symbol: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginBorrow {
    pub fn new(asset: &str, amount: Decimal) -> Self {
        Self {
            asset: asset.to_owned(),
            amount,
            is_isolated: None,
            symbol: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn is_isolated(mut self, is_isolated: bool) -> Self {
        self.is_isolated = Some(is_isolated);
        self
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
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

impl From<MarginBorrow> for Request {
    fn from(request: MarginBorrow) -> Request {
        let mut params = vec![
            ("asset".to_owned(), request.asset.to_string()),
            ("amount".to_owned(), request.amount.to_string()),
        ];

        if let Some(is_isolated) = request.is_isolated {
            params.push((
                "isIsolated".to_owned(),
                is_isolated.to_string().to_uppercase(),
            ));
        }

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/loan".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginBorrow;
    use crate::http::{request::Request, Credentials, Method};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_borrow_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginBorrow::new("BTC", dec!(1.01))
            .symbol("BNBUSDT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/loan".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("asset".to_owned(), "BTC".to_string()),
                    ("amount".to_owned(), "1.01".to_string()),
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
