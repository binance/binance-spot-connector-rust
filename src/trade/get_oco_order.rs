use crate::http::{request::Request, Credentials, Method};

/// `GET /api/v3/orderList`
///
/// Retrieves a specific OCO based on provided optional parameters
///
/// Weight(IP): 2
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade;
///
/// let request = trade::get_oco_order().order_list_id(11);
/// ```
pub struct GetOCOOrder {
    order_list_id: Option<u64>,
    orig_client_order_id: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl GetOCOOrder {
    pub fn new() -> Self {
        Self {
            order_list_id: None,
            orig_client_order_id: None,
            recv_window: None,
            credentials: None,
        }
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

impl From<GetOCOOrder> for Request {
    fn from(request: GetOCOOrder) -> Request {
        let mut params = vec![];

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
            path: "/api/v3/orderList".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for GetOCOOrder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::GetOCOOrder;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_get_oco_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = GetOCOOrder::new()
            .order_list_id(11)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/orderList".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("orderListId".to_owned(), "11".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
