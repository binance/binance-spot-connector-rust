use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/margin/orderList`
///
/// Retrieves a specific OCO based on provided optional parameters
///
/// * Either `orderListId` or `origClientOrderId` must be provided
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::margin_oco_order().symbol("BNBUSDT").order_list_id(27);
/// ```
pub struct MarginOCOOrder {
    is_isolated: Option<bool>,
    symbol: Option<String>,
    order_list_id: Option<u64>,
    orig_client_order_id: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginOCOOrder {
    pub fn new() -> Self {
        Self {
            is_isolated: None,
            symbol: None,
            order_list_id: None,
            orig_client_order_id: None,
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

    pub fn order_list_id(mut self, order_list_id: u64) -> Self {
        self.order_list_id = Some(order_list_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id.to_owned());
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

impl Default for MarginOCOOrder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MarginOCOOrder> for Request {
    fn from(request: MarginOCOOrder) -> Request {
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

        if let Some(order_list_id) = request.order_list_id {
            params.push(("orderListId".to_owned(), order_list_id.to_string()));
        }

        if let Some(orig_client_order_id) = request.orig_client_order_id {
            params.push(("origClientOrderId".to_owned(), orig_client_order_id));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/orderList".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginOCOOrder;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_oco_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginOCOOrder::new()
            .symbol("BNBUSDT")
            .order_list_id(27)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/orderList".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("orderListId".to_owned(), "27".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
