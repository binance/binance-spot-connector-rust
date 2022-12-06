use crate::http::{request::Request, Credentials, Method};

/// `PUT /sapi/v1/userDataStream/isolated`
///
/// Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. It's recommended to send a ping about every 30 minutes.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::isolated_margin_stream;
///
/// let request = isolated_margin_stream::renew_listen_key("BTCUSDT", "listen-key");
/// ```
pub struct RenewListenKey {
    symbol: String,
    listen_key: String,
    credentials: Option<Credentials>,
}

impl RenewListenKey {
    pub fn new(symbol: &str, listen_key: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            listen_key: listen_key.to_owned(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<RenewListenKey> for Request {
    fn from(request: RenewListenKey) -> Request {
        let params = vec![
            ("symbol".to_owned(), request.symbol.to_string()),
            ("listenKey".to_owned(), request.listen_key.to_string()),
        ];

        Request {
            path: "/sapi/v1/userDataStream/isolated".to_owned(),
            method: Method::Put,
            params,
            credentials: request.credentials,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RenewListenKey;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn isolated_margin_stream_renew_listen_key_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = RenewListenKey::new("BTCUSDT", "listen-key")
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/userDataStream/isolated".to_owned(),
                credentials: Some(credentials),
                method: Method::Put,
                params: vec![
                    ("symbol".to_owned(), "BTCUSDT".to_string()),
                    ("listenKey".to_owned(), "listen-key".to_string()),
                ],
                sign: false
            }
        );
    }
}
