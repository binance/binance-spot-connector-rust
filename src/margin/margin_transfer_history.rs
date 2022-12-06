use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/transfer`
///
/// * Response in descending order
/// * Returns data for last 7 days by default
/// * Set `archived` to `true` to query data from 6 months ago
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_transfer_history().asset("BNB").current(1).size(100);
/// ```
pub struct MarginTransferHistory {
    asset: Option<String>,
    r#type: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    current: Option<u32>,
    size: Option<u32>,
    archived: Option<bool>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginTransferHistory {
    pub fn new() -> Self {
        Self {
            asset: None,
            r#type: None,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            archived: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
        self
    }

    pub fn r#type(mut self, r#type: &str) -> Self {
        self.r#type = Some(r#type.to_owned());
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

    pub fn current(mut self, current: u32) -> Self {
        self.current = Some(current);
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
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

impl From<MarginTransferHistory> for Request {
    fn from(request: MarginTransferHistory) -> Request {
        let mut params = vec![];

        if let Some(asset) = request.asset {
            params.push(("asset".to_owned(), asset));
        }

        if let Some(r#type) = request.r#type {
            params.push(("type".to_owned(), r#type));
        }

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(current) = request.current {
            params.push(("current".to_owned(), current.to_string()));
        }

        if let Some(size) = request.size {
            params.push(("size".to_owned(), size.to_string()));
        }

        if let Some(archived) = request.archived {
            params.push(("archived".to_owned(), archived.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/transfer".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for MarginTransferHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MarginTransferHistory;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_transfer_history_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginTransferHistory::new()
            .asset("BNB")
            .current(1)
            .size(100)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/transfer".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("asset".to_owned(), "BNB".to_string()),
                    ("current".to_owned(), "1".to_string()),
                    ("size".to_owned(), "100".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
