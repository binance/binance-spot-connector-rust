use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/account/disableFastWithdrawSwitch`
///
/// * This request will disable fastwithdraw switch under your account.
/// * You need to enable "trade" option for the api key which requests this endpoint.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::disable_fast_withdraw();
/// ```
pub struct DisableFastWithdraw {
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl DisableFastWithdraw {
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

impl From<DisableFastWithdraw> for Request {
    fn from(request: DisableFastWithdraw) -> Request {
        let mut params = vec![];

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/account/disableFastWithdrawSwitch".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for DisableFastWithdraw {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::DisableFastWithdraw;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_disable_fast_withdraw_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = DisableFastWithdraw::new()
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/account/disableFastWithdrawSwitch".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![("recvWindow".to_owned(), "5000".to_string()),],
                sign: true
            }
        );
    }
}
