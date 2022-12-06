use crate::http::{request::Request, Credentials, Method};
use rust_decimal::Decimal;

/// `POST /sapi/v1/margin/transfer`
///
/// Execute transfer between spot account and cross margin account.
///
/// Weight(IP): 600
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
/// use rust_decimal_macros::dec;
///
/// let request = margin::margin_transfer("BTC", dec!(1.01), 1);
/// ```
pub struct MarginTransfer {
    asset: String,
    amount: Decimal,
    r#type: u32,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginTransfer {
    pub fn new(asset: &str, amount: Decimal, r#type: u32) -> Self {
        Self {
            asset: asset.to_owned(),
            amount,
            r#type,
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

impl From<MarginTransfer> for Request {
    fn from(request: MarginTransfer) -> Request {
        let mut params = vec![
            ("asset".to_owned(), request.asset.to_string()),
            ("amount".to_owned(), request.amount.to_string()),
            ("type".to_owned(), request.r#type.to_string()),
        ];

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/transfer".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginTransfer;
    use crate::http::{request::Request, Credentials, Method};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_transfer_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginTransfer::new("BTC", dec!(1.01), 1)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/transfer".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("asset".to_owned(), "BTC".to_string()),
                    ("amount".to_owned(), "1.01".to_string()),
                    ("type".to_owned(), "1".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
