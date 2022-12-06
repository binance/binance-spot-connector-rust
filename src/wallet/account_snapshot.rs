use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/accountSnapshot`
///
/// * The query time period must be less than 30 days
/// * Support query within the last one month only
/// * If startTimeand endTime not sent, return records of the last 7 days by default
///
/// Weight(IP): 2400
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::account_snapshot("SPOT");
/// ```
pub struct AccountSnapshot {
    r#type: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl AccountSnapshot {
    pub fn new(r#type: &str) -> Self {
        Self {
            r#type: r#type.to_owned(),
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
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

impl From<AccountSnapshot> for Request {
    fn from(request: AccountSnapshot) -> Request {
        let mut params = vec![("type".to_owned(), request.r#type.to_string())];

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/accountSnapshot".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AccountSnapshot;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_account_snapshot_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = AccountSnapshot::new("SPOT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/accountSnapshot".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("type".to_owned(), "SPOT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
