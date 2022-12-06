use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/asset/assetDetail`
///
/// Fetch details of assets supported on Binance.
///
/// * Please get network and other deposit or withdraw details from `GET /sapi/v1/capital/config/getall`.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::asset_detail().asset("BNB");
/// ```
pub struct AssetDetail {
    asset: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl AssetDetail {
    pub fn new() -> Self {
        Self {
            asset: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
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

impl Default for AssetDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl From<AssetDetail> for Request {
    fn from(request: AssetDetail) -> Request {
        let mut params = vec![];

        if let Some(asset) = request.asset {
            params.push(("asset".to_owned(), asset));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/asset/assetDetail".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AssetDetail;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_asset_detail_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = AssetDetail::new()
            .asset("BNB")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/asset/assetDetail".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("asset".to_owned(), "BNB".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
