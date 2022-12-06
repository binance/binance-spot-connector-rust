use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/asset/get-funding-asset`
///
/// * Currently supports querying the following business assets：Binance Pay, Binance Card, Binance Gift Card, Stock Token
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::funding_wallet().asset("BNB").need_btc_valuation(true);
/// ```
pub struct FundingWallet {
    asset: Option<String>,
    need_btc_valuation: Option<bool>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl FundingWallet {
    pub fn new() -> Self {
        Self {
            asset: None,
            need_btc_valuation: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
        self
    }

    pub fn need_btc_valuation(mut self, need_btc_valuation: bool) -> Self {
        self.need_btc_valuation = Some(need_btc_valuation);
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

impl Default for FundingWallet {
    fn default() -> Self {
        Self::new()
    }
}

impl From<FundingWallet> for Request {
    fn from(request: FundingWallet) -> Request {
        let mut params = vec![];

        if let Some(asset) = request.asset {
            params.push(("asset".to_owned(), asset));
        }

        if let Some(need_btc_valuation) = request.need_btc_valuation {
            params.push((
                "needBtcValuation".to_owned(),
                need_btc_valuation.to_string(),
            ));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/asset/get-funding-asset".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FundingWallet;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_funding_wallet_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = FundingWallet::new()
            .asset("BNB")
            .need_btc_valuation(true)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/asset/get-funding-asset".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("asset".to_owned(), "BNB".to_string()),
                    ("needBtcValuation".to_owned(), "true".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
