use crate::http::{request::Request, Credentials, Method};

/// `PUT /api/v3/userDataStream`
///
/// Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. It's recommended to send a ping about every 30 minutes.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::stream;
///
/// let request = stream::renew_listen_key("listen-key");
/// ```
pub struct RenewListenKey {
    listen_key: String,
    credentials: Option<Credentials>,
}

impl RenewListenKey {
    pub fn new(listen_key: &str) -> Self {
        Self {
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
        let params = vec![("listenKey".to_owned(), request.listen_key.to_string())];

        Request {
            path: "/api/v3/userDataStream".to_owned(),
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
    fn stream_renew_listen_key_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = RenewListenKey::new("listen-key")
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/userDataStream".to_owned(),
                credentials: Some(credentials),
                method: Method::Put,
                params: vec![("listenKey".to_owned(), "listen-key".to_string()),],
                sign: false
            }
        );
    }
}
