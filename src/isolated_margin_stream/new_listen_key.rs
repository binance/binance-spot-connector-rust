use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/userDataStream/isolated`
///
/// Start a new user data stream.
/// The stream will close after 60 minutes unless a keepalive is sent. If the account has an active `listenKey`, that `listenKey` will be returned and its validity will be extended for 60 minutes.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::isolated_margin_stream;
///
/// let request = isolated_margin_stream::new_listen_key("BTCUSDT");
/// ```
pub struct NewListenKey {
    symbol: String,
    credentials: Option<Credentials>,
}

impl NewListenKey {
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

impl From<NewListenKey> for Request {
    fn from(request: NewListenKey) -> Request {
        let params = vec![("symbol".to_owned(), request.symbol.to_string())];

        Request {
            path: "/sapi/v1/userDataStream/isolated".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewListenKey;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn isolated_margin_stream_new_listen_key_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = NewListenKey::new("BTCUSDT")
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/userDataStream/isolated".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![("symbol".to_owned(), "BTCUSDT".to_string()),],
                sign: false
            }
        );
    }
}
