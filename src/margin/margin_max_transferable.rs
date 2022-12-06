use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/maxTransferable`
///
/// * If `isolatedSymbol` is not sent, crossed margin data will be sent.
///
/// Weight(IP): 50
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_max_transferable("BTC");
/// ```
pub struct MarginMaxTransferable {
    asset: String,
    isolated_symbol: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginMaxTransferable {
    pub fn new(asset: &str) -> Self {
        Self {
            asset: asset.to_owned(),
            isolated_symbol: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn isolated_symbol(mut self, isolated_symbol: &str) -> Self {
        self.isolated_symbol = Some(isolated_symbol.to_owned());
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

impl From<MarginMaxTransferable> for Request {
    fn from(request: MarginMaxTransferable) -> Request {
        let mut params = vec![("asset".to_owned(), request.asset.to_string())];

        if let Some(isolated_symbol) = request.isolated_symbol {
            params.push(("isolatedSymbol".to_owned(), isolated_symbol));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/maxTransferable".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginMaxTransferable;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_max_transferable_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginMaxTransferable::new("BTC")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/maxTransferable".to_owned(),
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
