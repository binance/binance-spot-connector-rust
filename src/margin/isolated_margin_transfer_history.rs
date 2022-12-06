use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/isolated/transfer`
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::isolated_margin_transfer_history("BNBUSDT").asset("BNB").trans_from("SPOT").trans_to("ISOLATED_MARGIN").current(1).size(100);
/// ```
pub struct IsolatedMarginTransferHistory {
    symbol: String,
    asset: Option<String>,
    trans_from: Option<String>,
    trans_to: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    current: Option<u64>,
    size: Option<u64>,
    archived: Option<bool>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl IsolatedMarginTransferHistory {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            asset: None,
            trans_from: None,
            trans_to: None,
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

    pub fn trans_from(mut self, trans_from: &str) -> Self {
        self.trans_from = Some(trans_from.to_owned());
        self
    }

    pub fn trans_to(mut self, trans_to: &str) -> Self {
        self.trans_to = Some(trans_to.to_owned());
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

impl From<IsolatedMarginTransferHistory> for Request {
    fn from(request: IsolatedMarginTransferHistory) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(asset) = request.asset {
            params.push(("asset".to_owned(), asset));
        }

        if let Some(trans_from) = request.trans_from {
            params.push(("transFrom".to_owned(), trans_from));
        }

        if let Some(trans_to) = request.trans_to {
            params.push(("transTo".to_owned(), trans_to));
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
            path: "/sapi/v1/margin/isolated/transfer".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsolatedMarginTransferHistory;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_isolated_margin_transfer_history_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = IsolatedMarginTransferHistory::new("BNBUSDT")
            .asset("BNB")
            .trans_from("SPOT")
            .trans_to("ISOLATED_MARGIN")
            .current(1)
            .size(100)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/isolated/transfer".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("asset".to_owned(), "BNB".to_string()),
                    ("transFrom".to_owned(), "SPOT".to_string()),
                    ("transTo".to_owned(), "ISOLATED_MARGIN".to_string()),
                    ("current".to_owned(), "1".to_string()),
                    ("size".to_owned(), "100".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
