use crate::http::{request::Request, Credentials, Method};
use rust_decimal::Decimal;

/// `POST /sapi/v1/margin/order/oco`
///
/// Send in a new OCO for a margin account
///
/// * Price Restrictions:
///   - SELL: Limit Price &gt; Last Price &gt; Stop Price
///   - BUY: Limit Price &lt; Last Price &lt; Stop Price
/// * Quantity Restrictions:
///   - Both legs must have the same quantity
///   - ICEBERG quantities however do not have to be the same.
/// * Order Rate Limit
///   - OCO counts as 2 orders against the order rate limit.
///
/// Weight(UID): 6
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
/// use rust_decimal_macros::dec;
///
/// let request = margin::margin_new_oco_order("BNBUSDT", "SELL", dec!(0.1), dec!(400.15), dec!(390.3)).stop_limit_price(dec!(290)).stop_limit_time_in_force("GTC");
/// ```
pub struct MarginNewOCOOrder {
    symbol: String,
    side: String,
    quantity: Decimal,
    price: Decimal,
    stop_price: Decimal,
    is_isolated: Option<bool>,
    list_client_order_id: Option<String>,
    limit_client_order_id: Option<String>,
    limit_iceberg_qty: Option<Decimal>,
    stop_client_order_id: Option<String>,
    stop_limit_price: Option<Decimal>,
    stop_iceberg_qty: Option<Decimal>,
    stop_limit_time_in_force: Option<String>,
    new_order_resp_type: Option<String>,
    side_effect_type: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginNewOCOOrder {
    pub fn new(
        symbol: &str,
        side: &str,
        quantity: Decimal,
        price: Decimal,
        stop_price: Decimal,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side: side.to_owned(),
            quantity,
            price,
            stop_price,
            is_isolated: None,
            list_client_order_id: None,
            limit_client_order_id: None,
            limit_iceberg_qty: None,
            stop_client_order_id: None,
            stop_limit_price: None,
            stop_iceberg_qty: None,
            stop_limit_time_in_force: None,
            new_order_resp_type: None,
            side_effect_type: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn is_isolated(mut self, is_isolated: bool) -> Self {
        self.is_isolated = Some(is_isolated);
        self
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    pub fn limit_client_order_id(mut self, limit_client_order_id: &str) -> Self {
        self.limit_client_order_id = Some(limit_client_order_id.to_owned());
        self
    }

    pub fn limit_iceberg_qty(mut self, limit_iceberg_qty: Decimal) -> Self {
        self.limit_iceberg_qty = Some(limit_iceberg_qty);
        self
    }

    pub fn stop_client_order_id(mut self, stop_client_order_id: &str) -> Self {
        self.stop_client_order_id = Some(stop_client_order_id.to_owned());
        self
    }

    pub fn stop_limit_price(mut self, stop_limit_price: Decimal) -> Self {
        self.stop_limit_price = Some(stop_limit_price);
        self
    }

    pub fn stop_iceberg_qty(mut self, stop_iceberg_qty: Decimal) -> Self {
        self.stop_iceberg_qty = Some(stop_iceberg_qty);
        self
    }

    pub fn stop_limit_time_in_force(mut self, stop_limit_time_in_force: &str) -> Self {
        self.stop_limit_time_in_force = Some(stop_limit_time_in_force.to_owned());
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: &str) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type.to_owned());
        self
    }

    pub fn side_effect_type(mut self, side_effect_type: &str) -> Self {
        self.side_effect_type = Some(side_effect_type.to_owned());
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

impl From<MarginNewOCOOrder> for Request {
    fn from(request: MarginNewOCOOrder) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol.to_string()),
            ("side".to_owned(), request.side.to_string()),
            ("quantity".to_owned(), request.quantity.to_string()),
            ("price".to_owned(), request.price.to_string()),
            ("stopPrice".to_owned(), request.stop_price.to_string()),
        ];

        if let Some(is_isolated) = request.is_isolated {
            params.push((
                "isIsolated".to_owned(),
                is_isolated.to_string().to_uppercase(),
            ));
        }

        if let Some(list_client_order_id) = request.list_client_order_id {
            params.push(("listClientOrderId".to_owned(), list_client_order_id));
        }

        if let Some(limit_client_order_id) = request.limit_client_order_id {
            params.push(("limitClientOrderId".to_owned(), limit_client_order_id));
        }

        if let Some(limit_iceberg_qty) = request.limit_iceberg_qty {
            params.push(("limitIcebergQty".to_owned(), limit_iceberg_qty.to_string()));
        }

        if let Some(stop_client_order_id) = request.stop_client_order_id {
            params.push(("stopClientOrderId".to_owned(), stop_client_order_id));
        }

        if let Some(stop_limit_price) = request.stop_limit_price {
            params.push(("stopLimitPrice".to_owned(), stop_limit_price.to_string()));
        }

        if let Some(stop_iceberg_qty) = request.stop_iceberg_qty {
            params.push(("stopIcebergQty".to_owned(), stop_iceberg_qty.to_string()));
        }

        if let Some(stop_limit_time_in_force) = request.stop_limit_time_in_force {
            params.push(("stopLimitTimeInForce".to_owned(), stop_limit_time_in_force));
        }

        if let Some(new_order_resp_type) = request.new_order_resp_type {
            params.push(("newOrderRespType".to_owned(), new_order_resp_type));
        }

        if let Some(side_effect_type) = request.side_effect_type {
            params.push(("sideEffectType".to_owned(), side_effect_type));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/order/oco".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginNewOCOOrder;
    use crate::http::{request::Request, Credentials, Method};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_new_oco_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request =
            MarginNewOCOOrder::new("BNBUSDT", "SELL", dec!(0.1), dec!(400.15), dec!(390.3))
                .stop_limit_price(dec!(290))
                .stop_limit_time_in_force("GTC")
                .recv_window(5000)
                .credentials(&credentials)
                .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/order/oco".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("side".to_owned(), "SELL".to_string()),
                    ("quantity".to_owned(), "0.1".to_string()),
                    ("price".to_owned(), "400.15".to_string()),
                    ("stopPrice".to_owned(), "390.3".to_string()),
                    ("stopLimitPrice".to_owned(), "290".to_string()),
                    ("stopLimitTimeInForce".to_owned(), "GTC".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
