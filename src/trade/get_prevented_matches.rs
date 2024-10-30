use crate::http::{request::Request, Credentials, Method};

/// `GET /api/v3/myPreventedMatches`
///
/// Displays the list of orders that were expired due to STP.
///
/// These are the combinations supported:
///
/// Weight(IP):
/// * If `symbol` is invalid: `2`;
/// * Querying by `preventedMatchId`: `2`;
/// * Querying by `orderId`: `20`;
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade;
///
/// let request = trade::get_prevented_matches("BNBUSDT");
/// ```
pub struct GetPreventedMatches {
    symbol: String,
    prevented_match_id: Option<u64>,
    order_id: Option<u64>,
    from_prevented_match_id: Option<u64>,
    limit: Option<u32>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl GetPreventedMatches {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            prevented_match_id: None,
            order_id: None,
            from_prevented_match_id: None,
            limit: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn prevented_match_id(mut self, prevented_match_id: u64) -> Self {
        self.prevented_match_id = Some(prevented_match_id);
        self
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn from_prevented_match_id(mut self, from_prevented_match_id: u64) -> Self {
        self.from_prevented_match_id = Some(from_prevented_match_id);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
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

impl From<GetPreventedMatches> for Request {
    fn from(request: GetPreventedMatches) -> Request {
        let mut params = vec![];

        params.push(("symbol".to_owned(), request.symbol));

        if let Some(prevented_match_id) = request.prevented_match_id {
            params.push((
                "preventedMatchId".to_owned(),
                prevented_match_id.to_string(),
            ));
        }

        if let Some(order_id) = request.order_id {
            params.push(("orderId".to_owned(), order_id.to_string()));
        }

        if let Some(from_prevented_match_id) = request.from_prevented_match_id {
            params.push((
                "fromPreventedMatchId".to_owned(),
                from_prevented_match_id.to_string(),
            ));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/myPreventedMatches".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GetPreventedMatches;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_get_prevented_matches_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = GetPreventedMatches::new("BNBUSDT")
            .order_id(11)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/myPreventedMatches".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("orderId".to_owned(), "11".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
