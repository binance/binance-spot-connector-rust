use crate::http::{request::Request, Credentials, Method};
use rust_decimal::Decimal;

/// `POST /sapi/v1/margin/isolated/transfer`
///
/// Weight(UID): 600
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
/// use rust_decimal_macros::dec;
///
/// let request = margin::isolated_margin_transfer("BTC", "BNBUSDT", "SPOT", "ISOLATED_MARGIN", dec!(1.01));
/// ```
pub struct IsolatedMarginTransfer {
    asset: String,
    symbol: String,
    trans_from: String,
    trans_to: String,
    amount: Decimal,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl IsolatedMarginTransfer {
    pub fn new(
        asset: &str,
        symbol: &str,
        trans_from: &str,
        trans_to: &str,
        amount: Decimal,
    ) -> Self {
        Self {
            asset: asset.to_owned(),
            symbol: symbol.to_owned(),
            trans_from: trans_from.to_owned(),
            trans_to: trans_to.to_owned(),
            amount,
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

impl From<IsolatedMarginTransfer> for Request {
    fn from(request: IsolatedMarginTransfer) -> Request {
        let mut params = vec![
            ("asset".to_owned(), request.asset.to_string()),
            ("symbol".to_owned(), request.symbol.to_string()),
            ("transFrom".to_owned(), request.trans_from.to_string()),
            ("transTo".to_owned(), request.trans_to.to_string()),
            ("amount".to_owned(), request.amount.to_string()),
        ];

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/isolated/transfer".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsolatedMarginTransfer;
    use crate::http::{request::Request, Credentials, Method};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_isolated_margin_transfer_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request =
            IsolatedMarginTransfer::new("BTC", "BNBUSDT", "SPOT", "ISOLATED_MARGIN", dec!(1.01))
                .recv_window(5000)
                .credentials(&credentials)
                .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/isolated/transfer".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("asset".to_owned(), "BTC".to_string()),
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("transFrom".to_owned(), "SPOT".to_string()),
                    ("transTo".to_owned(), "ISOLATED_MARGIN".to_string()),
                    ("amount".to_owned(), "1.01".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
