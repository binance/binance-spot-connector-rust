use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/asset/dust`
///
/// Convert dust assets to BNB.
///
/// Weight(UID): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::dust_transfer(vec!["BTC","USDT"]);
/// ```
pub struct DustTransfer {
    asset: Vec<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl DustTransfer {
    pub fn new(asset: Vec<&str>) -> Self {
        Self {
            asset: asset.iter().map(|s| s.to_string()).collect(),
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

impl From<DustTransfer> for Request {
    fn from(request: DustTransfer) -> Request {
        let mut params: Vec<(String, String)> = request
            .asset
            .iter()
            .map(|asset| ("asset".to_owned(), asset.to_string()))
            .collect();

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/asset/dust".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DustTransfer;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_dust_transfer_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = DustTransfer::new(vec!["BTC", "USDT"])
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/asset/dust".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("asset".to_owned(), "BTC".to_string()),
                    ("asset".to_owned(), "USDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
