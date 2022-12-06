use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/capital/withdraw/history`
///
/// Fetch withdraw history.
///
/// * `network` may not be in the response for old withdraw.
/// * Please notice the default `startTime` and `endTime` to make sure that time interval is within 0-90 days.
/// * If both `startTime` and `endTime` are sent, time between `startTime` and `endTime` must be less than 90 days
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::withdraw_history().coin("BNB").limit(500);
/// ```
pub struct WithdrawHistory {
    coin: Option<String>,
    withdraw_order_id: Option<String>,
    status: Option<u32>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    offset: Option<u32>,
    limit: Option<u32>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl WithdrawHistory {
    pub fn new() -> Self {
        Self {
            coin: None,
            withdraw_order_id: None,
            status: None,
            start_time: None,
            end_time: None,
            offset: None,
            limit: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn coin(mut self, coin: &str) -> Self {
        self.coin = Some(coin.to_owned());
        self
    }

    pub fn withdraw_order_id(mut self, withdraw_order_id: &str) -> Self {
        self.withdraw_order_id = Some(withdraw_order_id.to_owned());
        self
    }

    pub fn status(mut self, status: u32) -> Self {
        self.status = Some(status);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
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

impl From<WithdrawHistory> for Request {
    fn from(request: WithdrawHistory) -> Request {
        let mut params = vec![];

        if let Some(coin) = request.coin {
            params.push(("coin".to_owned(), coin));
        }

        if let Some(withdraw_order_id) = request.withdraw_order_id {
            params.push(("withdrawOrderId".to_owned(), withdraw_order_id));
        }

        if let Some(status) = request.status {
            params.push(("status".to_owned(), status.to_string()));
        }

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(offset) = request.offset {
            params.push(("offset".to_owned(), offset.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/capital/withdraw/history".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for WithdrawHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::WithdrawHistory;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_withdraw_history_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = WithdrawHistory::new()
            .coin("BNB")
            .limit(500)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/capital/withdraw/history".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("coin".to_owned(), "BNB".to_string()),
                    ("limit".to_owned(), "500".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
