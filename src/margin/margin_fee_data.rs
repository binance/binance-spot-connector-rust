use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/crossMarginData`
///
/// Get cross margin fee data collection with any vip level or user's current specific data as https://www.binance.com/en/margin-fee
///
/// Weight(IP): 1 when coin is specified; 5 when the coin parameter is omitted
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_fee_data().coin("BNB");
/// ```
pub struct MarginFeeData {
    vip_level: Option<u32>,
    coin: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginFeeData {
    pub fn new() -> Self {
        Self {
            vip_level: None,
            coin: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn vip_level(mut self, vip_level: u32) -> Self {
        self.vip_level = Some(vip_level);
        self
    }

    pub fn coin(mut self, coin: &str) -> Self {
        self.coin = Some(coin.to_owned());
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

impl Default for MarginFeeData {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MarginFeeData> for Request {
    fn from(request: MarginFeeData) -> Request {
        let mut params = vec![];

        if let Some(vip_level) = request.vip_level {
            params.push(("vipLevel".to_owned(), vip_level.to_string()));
        }

        if let Some(coin) = request.coin {
            params.push(("coin".to_owned(), coin));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/crossMarginData".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginFeeData;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_fee_data_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginFeeData::new()
            .coin("BNB")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/crossMarginData".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("coin".to_owned(), "BNB".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
