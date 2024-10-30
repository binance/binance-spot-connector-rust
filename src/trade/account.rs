use crate::http::{request::Request, Credentials, Method};

/// `GET /api/v3/account`
///
/// Get current account information.
///
/// Weight(IP): 20
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade;
///
/// let request = trade::account();
/// ```
pub struct Account {
    omit_zero_balances: Option<bool>,
    recv_window: Option<i64>,
    credentials: Option<Credentials>,
}

impl Account {
    pub fn new() -> Self {
        Self {
            omit_zero_balances: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn omit_zero_balances(mut self, omit_zero_balances: bool) -> Self {
        self.omit_zero_balances = Some(omit_zero_balances);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<Account> for Request {
    fn from(request: Account) -> Request {
        let mut params = vec![];

        if let Some(omit_zero_balances) = request.omit_zero_balances {
            params.push((
                "omitZeroBalances".to_owned(),
                omit_zero_balances.to_string(),
            ));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/account".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Account;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_account_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = Account::new()
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/account".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![("recvWindow".to_owned(), "5000".to_string()),],
                sign: true
            }
        );
    }
}
