use crate::http::{request::Request, Credentials, Method};

/// `POST /api/v3/userDataStream`
///
/// Start a new user data stream.
/// The stream will close after 60 minutes unless a keepalive is sent. If the account has an active `listenKey`, that `listenKey` will be returned and its validity will be extended for 60 minutes.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::stream;
///
/// let request = stream::new_listen_key();
/// ```
pub struct NewListenKey {
    credentials: Option<Credentials>,
}

impl NewListenKey {
    pub fn new() -> Self {
        Self { credentials: None }
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<NewListenKey> for Request {
    fn from(_request: NewListenKey) -> Request {
        let params = vec![];

        Request {
            path: "/api/v3/userDataStream".to_owned(),
            method: Method::Post,
            params,
            credentials: _request.credentials,
            sign: false,
        }
    }
}

impl Default for NewListenKey {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::NewListenKey;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn stream_new_listen_key_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = NewListenKey::new().credentials(&credentials).into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/userDataStream".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![],
                sign: false
            }
        );
    }
}
