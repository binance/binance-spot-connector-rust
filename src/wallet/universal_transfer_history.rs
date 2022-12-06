use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/asset/transfer`
///
/// * `fromSymbol` must be sent when type are ISOLATEDMARGIN_MARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
/// * `toSymbol` must be sent when type are MARGIN_ISOLATEDMARGIN and ISOLATEDMARGIN_ISOLATEDMARGIN
/// * Support query within the last 6 months only
/// * If `startTime` and `endTime` not sent, return records of the last 7 days by default
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::universal_transfer_history("MAIN_UMFUTURE").start_time(1640995200000).end_time(1640995200000).current(1).size(100).from_symbol("BNBUSDT").to_symbol("BNBUSDT");
/// ```
pub struct UniversalTransferHistory {
    r#type: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    current: Option<u32>,
    size: Option<u32>,
    from_symbol: Option<String>,
    to_symbol: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl UniversalTransferHistory {
    pub fn new(r#type: &str) -> Self {
        Self {
            r#type: r#type.to_owned(),
            start_time: None,
            end_time: None,
            current: None,
            size: None,
            from_symbol: None,
            to_symbol: None,
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

    pub fn current(mut self, current: u32) -> Self {
        self.current = Some(current);
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn from_symbol(mut self, from_symbol: &str) -> Self {
        self.from_symbol = Some(from_symbol.to_owned());
        self
    }

    pub fn to_symbol(mut self, to_symbol: &str) -> Self {
        self.to_symbol = Some(to_symbol.to_owned());
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

impl From<UniversalTransferHistory> for Request {
    fn from(request: UniversalTransferHistory) -> Request {
        let mut params = vec![("type".to_owned(), request.r#type.to_string())];

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

        if let Some(from_symbol) = request.from_symbol {
            params.push(("fromSymbol".to_owned(), from_symbol));
        }

        if let Some(to_symbol) = request.to_symbol {
            params.push(("toSymbol".to_owned(), to_symbol));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/asset/transfer".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UniversalTransferHistory;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_universal_transfer_history_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = UniversalTransferHistory::new("MAIN_UMFUTURE")
            .start_time(1640995200000)
            .end_time(1640995200000)
            .current(1)
            .size(100)
            .from_symbol("BNBUSDT")
            .to_symbol("BNBUSDT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/asset/transfer".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("type".to_owned(), "MAIN_UMFUTURE".to_string()),
                    ("startTime".to_owned(), "1640995200000".to_string()),
                    ("endTime".to_owned(), "1640995200000".to_string()),
                    ("current".to_owned(), "1".to_string()),
                    ("size".to_owned(), "100".to_string()),
                    ("fromSymbol".to_owned(), "BNBUSDT".to_string()),
                    ("toSymbol".to_owned(), "BNBUSDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
