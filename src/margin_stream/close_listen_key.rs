use crate::http::{request::Request, Credentials, Method};

/// `DELETE /sapi/v1/userDataStream`
///
/// Close out a user data stream.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin_stream;
///
/// let request = margin_stream::close_listen_key("listen-key");
/// ```
pub struct CloseListenKey {
    listen_key: String,
    credentials: Option<Credentials>,
}

impl CloseListenKey {
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

impl From<CloseListenKey> for Request {
    fn from(request: CloseListenKey) -> Request {
        let params = vec![("listenKey".to_owned(), request.listen_key.to_string())];

        Request {
            path: "/sapi/v1/userDataStream".to_owned(),
            method: Method::Delete,
            params,
            credentials: request.credentials,
            sign: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CloseListenKey;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_stream_close_listen_key_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = CloseListenKey::new("listen-key")
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/userDataStream".to_owned(),
                credentials: Some(credentials),
                method: Method::Delete,
                params: vec![("listenKey".to_owned(), "listen-key".to_string()),],
                sign: false
            }
        );
    }
}
