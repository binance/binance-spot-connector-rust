use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/allPairs`
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_all_pairs();
/// ```
pub struct MarginAllPairs {
    credentials: Option<Credentials>,
}

impl MarginAllPairs {
    pub fn new() -> Self {
        Self { credentials: None }
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginAllPairs> for Request {
    fn from(_request: MarginAllPairs) -> Request {
        let params = vec![];

        Request {
            path: "/sapi/v1/margin/allPairs".to_owned(),
            method: Method::Get,
            params,
            credentials: _request.credentials,
            sign: false,
        }
    }
}

impl Default for MarginAllPairs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MarginAllPairs;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_all_pairs_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginAllPairs::new().credentials(&credentials).into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/allPairs".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![],
                sign: false
            }
        );
    }
}
