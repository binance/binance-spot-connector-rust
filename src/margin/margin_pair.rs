use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/pair`
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_pair("BNBUSDT");
/// ```
pub struct MarginPair {
    symbol: String,
    credentials: Option<Credentials>,
}

impl MarginPair {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginPair> for Request {
    fn from(request: MarginPair) -> Request {
        let params = vec![("symbol".to_owned(), request.symbol.to_string())];

        Request {
            path: "/sapi/v1/margin/pair".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginPair;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_pair_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginPair::new("BNBUSDT").credentials(&credentials).into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/pair".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![("symbol".to_owned(), "BNBUSDT".to_string()),],
                sign: false
            }
        );
    }
}
