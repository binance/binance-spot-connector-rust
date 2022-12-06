use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/isolatedMarginTier`
///
/// Get isolated margin tier data collection with any tier as https://www.binance.com/en/margin-data
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::isolated_margin_tier_data("BNBUSDT").tier("1");
/// ```
pub struct IsolatedMarginTierData {
    symbol: String,
    tier: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl IsolatedMarginTierData {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            tier: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn tier(mut self, tier: &str) -> Self {
        self.tier = Some(tier.to_owned());
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

impl From<IsolatedMarginTierData> for Request {
    fn from(request: IsolatedMarginTierData) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(tier) = request.tier {
            params.push(("tier".to_owned(), tier));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/isolatedMarginTier".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsolatedMarginTierData;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_isolated_margin_tier_data_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = IsolatedMarginTierData::new("BNBUSDT")
            .tier("1")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/isolatedMarginTier".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("tier".to_owned(), "1".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
