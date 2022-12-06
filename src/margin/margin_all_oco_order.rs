use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/allOrderList`
///
/// Retrieves all OCO for a specific margin account based on provided optional parameters
///
/// Weight(IP): 200
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_all_oco_order();
/// ```
pub struct MarginAllOCOOrder {
    is_isolated: Option<bool>,
    symbol: Option<String>,
    from_id: Option<u64>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginAllOCOOrder {
    pub fn new() -> Self {
        Self {
            is_isolated: None,
            symbol: None,
            from_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn is_isolated(mut self, is_isolated: bool) -> Self {
        self.is_isolated = Some(is_isolated);
        self
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
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

impl From<MarginAllOCOOrder> for Request {
    fn from(request: MarginAllOCOOrder) -> Request {
        let mut params = vec![];

        if let Some(is_isolated) = request.is_isolated {
            params.push((
                "isIsolated".to_owned(),
                is_isolated.to_string().to_uppercase(),
            ));
        }

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(from_id) = request.from_id {
            params.push(("fromId".to_owned(), from_id.to_string()));
        }

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/allOrderList".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for MarginAllOCOOrder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::MarginAllOCOOrder;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_all_oco_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginAllOCOOrder::new()
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/allOrderList".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![("recvWindow".to_owned(), "5000".to_string()),],
                sign: true
            }
        );
    }
}
