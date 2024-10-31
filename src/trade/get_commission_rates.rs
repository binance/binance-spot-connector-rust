use crate::http::{request::Request, Credentials, Method};

/// `GET /api/v3/account/commission`
///
/// Get current account commission rates.
///
/// Weight(IP): 20
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade;
///
/// let request = trade::get_commission_rates("BNBUSDT");
/// ```
pub struct GetCommissionRates {
    symbol: String,
    credentials: Option<Credentials>,
}

impl GetCommissionRates {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<GetCommissionRates> for Request {
    fn from(request: GetCommissionRates) -> Request {
        let params = vec![("symbol".to_owned(), request.symbol.to_string())];

        Request {
            path: "/api/v3/account/commission".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GetCommissionRates;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_get_commission_rates_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = GetCommissionRates::new("BNBUSDT")
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/account/commission".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![("symbol".to_owned(), "BNBUSDT".to_string())],
                sign: true
            }
        );
    }
}
