use crate::http::{request::Request, Credentials, Method};
use crate::trade::order::{NewOrderResponseType, Side, TimeInForce};
use rust_decimal::Decimal;

/// `POST /api/v3/order/test`
///
/// Test new order creation and signature/recvWindow long.
/// Creates and validates a new order but does not send it into the matching engine.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade::{self, order::Side};
/// use rust_decimal_macros::dec;
///
/// let request = trade::new_order_test("BNBUSDT", Side::Sell, "MARKET").stop_price(dec!(20.01));
/// ```
pub struct NewOrderTest {
    symbol: String,
    side: Side,
    r#type: String,
    time_in_force: Option<TimeInForce>,
    quantity: Option<Decimal>,
    quote_order_qty: Option<Decimal>,
    price: Option<Decimal>,
    new_client_order_id: Option<String>,
    stop_price: Option<Decimal>,
    trailing_delta: Option<Decimal>,
    iceberg_qty: Option<Decimal>,
    new_order_resp_type: Option<NewOrderResponseType>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl NewOrderTest {
    pub fn new(symbol: &str, side: Side, r#type: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type: r#type.to_owned(),
            time_in_force: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            trailing_delta: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
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

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn stop_price(mut self, stop_price: Decimal) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn trailing_delta(mut self, trailing_delta: Decimal) -> Self {
        self.trailing_delta = Some(trailing_delta);
        self
    }

    pub fn iceberg_qty(mut self, iceberg_qty: Decimal) -> Self {
        self.iceberg_qty = Some(iceberg_qty);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: NewOrderResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
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

impl From<NewOrderTest> for Request {
    fn from(request: NewOrderTest) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol.to_string()),
            ("side".to_owned(), request.side.to_string()),
            ("type".to_owned(), request.r#type.to_string()),
        ];

        if let Some(time_in_force) = request.time_in_force {
            params.push(("timeInForce".to_owned(), time_in_force.to_string()));
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

        if let Some(new_client_order_id) = request.new_client_order_id {
            params.push(("newClientOrderId".to_owned(), new_client_order_id));
        }

        if let Some(stop_price) = request.stop_price {
            params.push(("stopPrice".to_owned(), stop_price.to_string()));
        }

        if let Some(trailing_delta) = request.trailing_delta {
            params.push(("trailingDelta".to_owned(), trailing_delta.to_string()));
        }

        if let Some(iceberg_qty) = request.iceberg_qty {
            params.push(("icebergQty".to_owned(), iceberg_qty.to_string()));
        }

        if let Some(new_order_resp_type) = request.new_order_resp_type {
            params.push((
                "newOrderRespType".to_owned(),
                new_order_resp_type.to_string(),
            ));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/order/test".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewOrderTest;
    use crate::http::{request::Request, Credentials, Method};
    use crate::trade::order::Side;
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_new_order_test_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = NewOrderTest::new("BNBUSDT", Side::Sell, "MARKET")
            .stop_price(dec!(20.01))
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/order/test".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("side".to_owned(), "SELL".to_string()),
                    ("type".to_owned(), "MARKET".to_string()),
                    ("stopPrice".to_owned(), "20.01".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
