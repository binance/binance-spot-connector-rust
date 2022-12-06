use crate::http::{request::Request, Credentials, Method};
use rust_decimal::Decimal;

/// `POST /sapi/v1/margin/order`
///
/// Post a new order for margin account.
///
/// Weight(UID): 6
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
/// use rust_decimal_macros::dec;
///
/// let request = margin::margin_new_order("BNBUSDT", "SELL", "MARKET").quantity(dec!(1.01)).price(dec!(10)).stop_price(dec!(20.01)).time_in_force("GTC");
/// ```
pub struct MarginNewOrder {
    symbol: String,
    side: String,
    r#type: String,
    is_isolated: Option<bool>,
    quantity: Option<Decimal>,
    quote_order_qty: Option<Decimal>,
    price: Option<Decimal>,
    stop_price: Option<Decimal>,
    new_client_order_id: Option<String>,
    iceberg_qty: Option<Decimal>,
    new_order_resp_type: Option<String>,
    side_effect_type: Option<String>,
    time_in_force: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl MarginNewOrder {
    pub fn new(symbol: &str, side: &str, r#type: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side: side.to_owned(),
            r#type: r#type.to_owned(),
            is_isolated: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            stop_price: None,
            new_client_order_id: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            side_effect_type: None,
            time_in_force: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn is_isolated(mut self, is_isolated: bool) -> Self {
        self.is_isolated = Some(is_isolated);
        self
    }

    pub fn quantity(mut self, quantity: Decimal) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn quote_order_qty(mut self, quote_order_qty: Decimal) -> Self {
        self.quote_order_qty = Some(quote_order_qty);
        self
    }

    pub fn price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    pub fn stop_price(mut self, stop_price: Decimal) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn iceberg_qty(mut self, iceberg_qty: Decimal) -> Self {
        self.iceberg_qty = Some(iceberg_qty);
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

    pub fn time_in_force(mut self, time_in_force: &str) -> Self {
        self.time_in_force = Some(time_in_force.to_owned());
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

impl From<MarginNewOrder> for Request {
    fn from(request: MarginNewOrder) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol.to_string()),
            ("side".to_owned(), request.side.to_string()),
            ("type".to_owned(), request.r#type.to_string()),
        ];

        if let Some(is_isolated) = request.is_isolated {
            params.push((
                "isIsolated".to_owned(),
                is_isolated.to_string().to_uppercase(),
            ));
        }

        if let Some(quantity) = request.quantity {
            params.push(("quantity".to_owned(), quantity.to_string()));
        }

        if let Some(quote_order_qty) = request.quote_order_qty {
            params.push(("quoteOrderQty".to_owned(), quote_order_qty.to_string()));
        }

        if let Some(price) = request.price {
            params.push(("price".to_owned(), price.to_string()));
        }

        if let Some(stop_price) = request.stop_price {
            params.push(("stopPrice".to_owned(), stop_price.to_string()));
        }

        if let Some(new_client_order_id) = request.new_client_order_id {
            params.push(("newClientOrderId".to_owned(), new_client_order_id));
        }

        if let Some(iceberg_qty) = request.iceberg_qty {
            params.push(("icebergQty".to_owned(), iceberg_qty.to_string()));
        }

        if let Some(new_order_resp_type) = request.new_order_resp_type {
            params.push(("newOrderRespType".to_owned(), new_order_resp_type));
        }

        if let Some(side_effect_type) = request.side_effect_type {
            params.push(("sideEffectType".to_owned(), side_effect_type));
        }

        if let Some(time_in_force) = request.time_in_force {
            params.push(("timeInForce".to_owned(), time_in_force));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/margin/order".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarginNewOrder;
    use crate::http::{request::Request, Credentials, Method};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_margin_new_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = MarginNewOrder::new("BNBUSDT", "SELL", "MARKET")
            .quantity(dec!(1.01))
            .price(dec!(10))
            .stop_price(dec!(20.01))
            .time_in_force("GTC")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/margin/order".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("side".to_owned(), "SELL".to_string()),
                    ("type".to_owned(), "MARKET".to_string()),
                    ("quantity".to_owned(), "1.01".to_string()),
                    ("price".to_owned(), "10".to_string()),
                    ("stopPrice".to_owned(), "20.01".to_string()),
                    ("timeInForce".to_owned(), "GTC".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
