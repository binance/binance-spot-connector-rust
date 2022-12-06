use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/isolated/account`
///
/// * If "symbols" is not sent, all isolated assets will be returned.
/// * If "symbols" is sent, only the isolated assets of the sent symbols will be returned.
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::isolated_margin_account().symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct IsolatedMarginAccount {
    symbols: Option<Vec<String>>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl IsolatedMarginAccount {
    pub fn new() -> Self {
        Self {
            symbols: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn symbols(mut self, symbols: Vec<&str>) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
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

impl From<IsolatedMarginAccount> for Request {
    fn from(request: IsolatedMarginAccount) -> Request {
        let mut params = vec![];

        if let Some(symbols) = request.symbols {
            params.push(("symbols".to_owned(), symbols.join(",")));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/isolated/account".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for IsolatedMarginAccount {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::IsolatedMarginAccount;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_isolated_margin_account_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = IsolatedMarginAccount::new()
            .symbols(vec!["BTCUSDT", "BNBBTC"])
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/isolated/account".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbols".to_owned(), "BTCUSDT,BNBBTC".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
