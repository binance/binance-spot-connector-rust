use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/asset/custody/transfer-history`
///
/// Query User Delegation History
///
/// * You need to open Enable Spot & Margin Trading permission for the API Key which requests this endpoint
///
/// Weight(IP): 60
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::transfer_history("a@a", 1695205406000, 1695208406000);
/// ```
pub struct TransferHistory {
    email: String,
    start_time: u64,
    end_time: u64,
    transfer_type: Option<String>,
    asset: Option<String>,
    current: Option<u32>,
    size: Option<u32>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl TransferHistory {
    pub fn new(email: &str, start_time: u64, end_time: u64) -> Self {
        Self {
            email: email.to_owned(),
            start_time,
            end_time,
            transfer_type: None,
            asset: None,
            current: None,
            size: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn transfer_type(mut self, transfer_type: &str) -> Self {
        self.transfer_type = Some(transfer_type.to_owned());
        self
    }

    pub fn asset(mut self, asset: &str) -> Self {
        self.asset = Some(asset.to_owned());
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

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<TransferHistory> for Request {
    fn from(request: TransferHistory) -> Request {
        let mut params = vec![
            ("email".to_owned(), request.email),
            ("startTime".to_owned(), request.start_time.to_string()),
            ("endTime".to_owned(), request.end_time.to_string()),
        ];

        if let Some(transfer_type) = request.transfer_type {
            params.push(("transferType".to_owned(), transfer_type));
        }

        if let Some(asset) = request.asset {
            params.push(("asset".to_owned(), asset));
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
            path: "/sapi/v1/asset/custody/transfer-history".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TransferHistory;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_transfert_history_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = TransferHistory::new("a@a", 1695205406000, 1695208406000)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/asset/custody/transfer-history".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("email".to_owned(), "a@a".to_string()),
                    ("startTime".to_owned(), "1695205406000".to_string()),
                    ("endTime".to_owned(), "1695208406000".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
