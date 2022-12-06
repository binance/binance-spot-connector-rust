use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/interestRateHistory`
///
/// The max interval between startTime and endTime is 30 days.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_interest_rate_history("BTC");
/// ```
pub struct MarginInterestRateHistory {
    asset: String,
    vip_level: Option<u32>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginInterestRateHistory {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            vip_level: None,
            start_time: None,
            end_time: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn vip_level(mut self, vip_level: u32) -> Self {
        self.vip_level = Some(vip_level);
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

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginInterestRateHistory> for Request {
    fn from(request: MarginInterestRateHistory) -> Request {
        let mut params = vec![("asset".to_owned(), request.asset.to_string())];

        if let Some(vip_level) = request.vip_level {
            params.push(("vipLevel".to_owned(), vip_level.to_string()));
        }

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/interestRateHistory".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginInterestRateHistory;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_interest_rate_history_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginInterestRateHistory::new("BTC")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/interestRateHistory".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("asset".to_owned(), "BTC".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
