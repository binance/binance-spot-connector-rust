use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/asset/wallet/balance`
///
/// Query User Wallet Balance
///
/// Weight(IP): 60
///
/// * You need to open Permits Universal Transfer permission for the API Key which requests this endpoint.
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::balance().recv_window(5000);
/// ```
pub struct Balance {
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl Balance {
    pub fn new() -> Self {
        Self {
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

impl From<Balance> for Request {
    fn from(request: Balance) -> Request {
        let mut params = vec![];

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/asset/wallet/balance".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for Balance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Balance;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_balance_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = Balance::new()
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/asset/wallet/balance".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![("recvWindow".to_owned(), "5000".to_string())],
                sign: true
            }
        );
    }
}
