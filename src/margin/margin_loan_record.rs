use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/loan`
///
/// * `txId` or `startTime` must be sent. `txId` takes precedence.
/// * Response in descending order
/// * If `isolatedSymbol` is not sent, crossed margin data will be returned
/// * Set `archived` to `true` to query data from 6 months ago
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_loan_record("BTC").tx_id(123456789).current(1).size(100);
/// ```
pub struct MarginLoanRecord {
    asset: String,
    isolated_symbol: Option<String>,
    tx_id: Option<u64>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    current: Option<u64>,
    size: Option<u64>,
    archived: Option<bool>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginLoanRecord {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            isolated_symbol: None,
            tx_id: None,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            archived: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn isolated_symbol(mut self, isolated_symbol: &str) -> Self {
        self.isolated_symbol = Some(isolated_symbol.to_owned());
        self
    }

    pub fn tx_id(mut self, tx_id: u64) -> Self {
        self.tx_id = Some(tx_id);
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

    pub fn current(mut self, current: u64) -> Self {
        self.current = Some(current);
        self
    }

    pub fn size(mut self, size: u64) -> Self {
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

impl From<MarginLoanRecord> for Request {
    fn from(request: MarginLoanRecord) -> Request {
        let mut params = vec![("asset".to_owned(), request.asset.to_string())];

        if let Some(isolated_symbol) = request.isolated_symbol {
            params.push(("isolatedSymbol".to_owned(), isolated_symbol));
        }

        if let Some(tx_id) = request.tx_id {
            params.push(("txId".to_owned(), tx_id.to_string()));
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
            path: "/sapi/v1/margin/loan".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginLoanRecord;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_loan_record_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginLoanRecord::new("BTC")
            .tx_id(123456789)
            .current(1)
            .size(100)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/loan".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("asset".to_owned(), "BTC".to_string()),
                    ("txId".to_owned(), "123456789".to_string()),
                    ("current".to_owned(), "1".to_string()),
                    ("size".to_owned(), "100".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
