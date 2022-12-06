use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/asset`
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_asset("BTC");
/// ```
pub struct MarginAsset {
    asset: String,
    credentials: Option<Credentials>,
}

impl MarginAsset {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginAsset> for Request {
    fn from(request: MarginAsset) -> Request {
        let params = vec![("asset".to_owned(), request.asset.to_string())];

        Request {
            path: "/sapi/v1/margin/asset".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginAsset;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_asset_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginAsset::new("BTC").credentials(&credentials).into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/asset".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![("asset".to_owned(), "BTC".to_string()),],
                sign: false
            }
        );
    }
}
