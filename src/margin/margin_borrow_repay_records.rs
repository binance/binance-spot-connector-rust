use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/borrow-repay`
///
/// Query borrow/repay records in Margin account
///
/// * txId or startTime must be sent. txId takes precedence. Response in descending order
/// * If an asset is sent, data within 30 days before endTime; If an asset is not sent, data within 7 days before endTime
/// * If neither startTime nor endTime is sent, the recent 7-day data will be returned.
/// * startTime set as endTime - 7days by default, endTime set as current time by default
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_borrow_repay_records("BORROW");
/// ```
pub struct MarginBorrowRepayRecords {
    type_: String,
    asset: Option<String>,
    isolated_symbol: Option<String>,
    tx_id: Option<u64>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    current: Option<u64>,
    size: Option<u64>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginBorrowRepayRecords {
    pub fn new(type_: &str) -> Self {
        Self {
            type_: type_.to_owned(),
            asset: None,
            isolated_symbol: None,
            tx_id: None,
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
        self
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

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<MarginBorrowRepayRecords> for Request {
    fn from(request: MarginBorrowRepayRecords) -> Request {
        let mut params = vec![("type".to_owned(), request.type_.to_string())];

        if let Some(asset) = request.asset {
            params.push(("asset".to_owned(), asset));
        }

        if let Some(isolated_symbol) = request.isolated_symbol {
            params.push(("isolated_symbol".to_owned(), isolated_symbol));
        }

        if let Some(tx_id) = request.tx_id {
            params.push(("tx_id".to_owned(), tx_id.to_string()));
        }

        if let Some(start_time) = request.start_time {
            params.push(("start_time".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("end_time".to_owned(), end_time.to_string()));
        }

        if let Some(current) = request.current {
            params.push(("current".to_owned(), current.to_string()));
        }

        if let Some(size) = request.size {
            params.push(("size".to_owned(), size.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/borrow-repay".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginBorrowRepayRecords;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_borrow_repay_records_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginBorrowRepayRecords::new("BORROW")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/borrow-repay".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("type".to_owned(), "BORROW".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
