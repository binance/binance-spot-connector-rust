use crate::http::{request::Request, Credentials, Method};
use rust_decimal::Decimal;

/// `POST /sapi/v1/asset/transfer`
///
/// You need to enable `Permits Universal Transfer` option for the api key which requests this endpoint.
///
/// * `fromSymbol` must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
/// * `toSymbol` must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
///
/// ENUM of transfer types:
/// * MAIN_UMFUTURE Spot account transfer to USDⓈ-M Futures account
/// * MAIN_CMFUTURE Spot account transfer to COIN-M Futures account
/// * MAIN_MARGIN Spot account transfer to Margin（cross）account
/// * UMFUTURE_MAIN USDⓈ-M Futures account transfer to Spot account
/// * UMFUTURE_MARGIN USDⓈ-M Futures account transfer to Margin（cross）account
/// * CMFUTURE_MAIN COIN-M Futures account transfer to Spot account
/// * CMFUTURE_MARGIN COIN-M Futures account transfer to Margin(cross) account
/// * MARGIN_MAIN Margin（cross）account transfer to Spot account
/// * MARGIN_UMFUTURE Margin（cross）account transfer to USDⓈ-M Futures
/// * MARGIN_CMFUTURE Margin（cross）account transfer to COIN-M Futures
/// * ISOLATEDMARGIN_MARGIN Isolated margin account transfer to Margin(cross) account
/// * MARGIN_ISOLATEDMARGIN Margin(cross) account transfer to Isolated margin account
/// * ISOLATEDMARGIN_ISOLATEDMARGIN Isolated margin account transfer to Isolated margin account
/// * MAIN_FUNDING Spot account transfer to Funding account
/// * FUNDING_MAIN Funding account transfer to Spot account
/// * FUNDING_UMFUTURE Funding account transfer to UMFUTURE account
/// * UMFUTURE_FUNDING UMFUTURE account transfer to Funding account
/// * MARGIN_FUNDING MARGIN account transfer to Funding account
/// * FUNDING_MARGIN Funding account transfer to Margin account
/// * FUNDING_CMFUTURE Funding account transfer to CMFUTURE account
/// * CMFUTURE_FUNDING CMFUTURE account transfer to Funding account
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
/// use rust_decimal_macros::dec;
///
/// let request = wallet::universal_transfer("MAIN_UMFUTURE", "BTC", dec!(1.01)).from_symbol("BNBUSDT").to_symbol("BNBUSDT");
/// ```
pub struct UniversalTransfer {
    r#type: String,
    asset: String,
    amount: Decimal,
    from_symbol: Option<String>,
    to_symbol: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl UniversalTransfer {
    pub fn new(r#type: &str, asset: &str, amount: Decimal) -> Self {
        Self {
            r#type: r#type.to_owned(),
            asset: asset.to_owned(),
            amount,
            from_symbol: None,
            to_symbol: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn from_symbol(mut self, from_symbol: &str) -> Self {
        self.from_symbol = Some(from_symbol.to_owned());
        self
    }

    pub fn to_symbol(mut self, to_symbol: &str) -> Self {
        self.to_symbol = Some(to_symbol.to_owned());
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

impl From<UniversalTransfer> for Request {
    fn from(request: UniversalTransfer) -> Request {
        let mut params = vec![
            ("type".to_owned(), request.r#type.to_string()),
            ("asset".to_owned(), request.asset.to_string()),
            ("amount".to_owned(), request.amount.to_string()),
        ];

        if let Some(from_symbol) = request.from_symbol {
            params.push(("fromSymbol".to_owned(), from_symbol));
        }

        if let Some(to_symbol) = request.to_symbol {
            params.push(("toSymbol".to_owned(), to_symbol));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/asset/transfer".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UniversalTransfer;
    use crate::http::{request::Request, Credentials, Method};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_universal_transfer_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = UniversalTransfer::new("MAIN_UMFUTURE", "BTC", dec!(1.01))
            .from_symbol("BNBUSDT")
            .to_symbol("BNBUSDT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/asset/transfer".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("type".to_owned(), "MAIN_UMFUTURE".to_string()),
                    ("asset".to_owned(), "BTC".to_string()),
                    ("amount".to_owned(), "1.01".to_string()),
                    ("fromSymbol".to_owned(), "BNBUSDT".to_string()),
                    ("toSymbol".to_owned(), "BNBUSDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
