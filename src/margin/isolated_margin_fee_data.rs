use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/isolatedMarginData`
///
/// Get isolated margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee
///
/// Weight(IP): 1 when a single is specified; 10 when the symbol parameter is omitted
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::isolated_margin_fee_data().symbol("BNBUSDT");
/// ```
pub struct IsolatedMarginFeeData {
    vip_level: Option<u32>,
    symbol: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl IsolatedMarginFeeData {
    pub fn new() -> Self {
        Self {
            vip_level: None,
            symbol: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn vip_level(mut self, vip_level: u32) -> Self {
        self.vip_level = Some(vip_level);
        self
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
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

impl From<IsolatedMarginFeeData> for Request {
    fn from(request: IsolatedMarginFeeData) -> Request {
        let mut params = vec![];

        if let Some(vip_level) = request.vip_level {
            params.push(("vipLevel".to_owned(), vip_level.to_string()));
        }

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/isolatedMarginData".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for IsolatedMarginFeeData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::IsolatedMarginFeeData;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_isolated_margin_fee_data_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = IsolatedMarginFeeData::new()
            .symbol("BNBUSDT")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/isolatedMarginData".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
