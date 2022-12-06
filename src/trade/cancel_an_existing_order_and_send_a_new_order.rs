use crate::http::{request::Request, Credentials, Method};
use crate::trade::order::{CancelReplaceMode, NewOrderResponseType, Side, TimeInForce};
use rust_decimal::Decimal;

/// `POST /api/v3/order/cancelReplace`
///
/// Cancels an existing order and places a new order on the same symbol.
///
/// Filters are evaluated before the cancel order is placed.
///
/// If the new order placement is successfully sent to the engine, the order count will increase by 1.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade::{self, order::{Side, TimeInForce, CancelReplaceMode}};
/// use rust_decimal_macros::dec;
///
/// let request = trade::cancel_an_existing_order_and_send_a_new_order("BNBUSDT", Side::Sell, "LIMIT", CancelReplaceMode::StopOnFailure).time_in_force(TimeInForce::Gtc).quantity(dec!(10.1)).price(dec!(295.92)).cancel_order_id(12).stop_price(dec!(20.01));
/// ```
pub struct CancelAnExistingOrderAndSendANewOrder {
    symbol: String,
    side: Side,
    r#type: String,
    cancel_replace_mode: CancelReplaceMode,
    time_in_force: Option<TimeInForce>,
    quantity: Option<Decimal>,
    quote_order_qty: Option<Decimal>,
    price: Option<Decimal>,
    cancel_new_client_order_id: Option<String>,
    cancel_orig_client_order_id: Option<String>,
    cancel_order_id: Option<u64>,
    new_client_order_id: Option<String>,
    stop_price: Option<Decimal>,
    trailing_delta: Option<Decimal>,
    iceberg_qty: Option<Decimal>,
    new_order_resp_type: Option<NewOrderResponseType>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl CancelAnExistingOrderAndSendANewOrder {
    pub fn new(
        symbol: &str,
        side: Side,
        r#type: &str,
        cancel_replace_mode: CancelReplaceMode,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            r#type: r#type.to_owned(),
            cancel_replace_mode,
            time_in_force: None,
            quantity: None,
            quote_order_qty: None,
            price: None,
            cancel_new_client_order_id: None,
            cancel_orig_client_order_id: None,
            cancel_order_id: None,
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

    pub fn cancel_new_client_order_id(mut self, cancel_new_client_order_id: &str) -> Self {
        self.cancel_new_client_order_id = Some(cancel_new_client_order_id.to_owned());
        self
    }

    pub fn cancel_orig_client_order_id(mut self, cancel_orig_client_order_id: &str) -> Self {
        self.cancel_orig_client_order_id = Some(cancel_orig_client_order_id.to_owned());
        self
    }

    pub fn cancel_order_id(mut self, cancel_order_id: u64) -> Self {
        self.cancel_order_id = Some(cancel_order_id);
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

impl From<CancelAnExistingOrderAndSendANewOrder> for Request {
    fn from(request: CancelAnExistingOrderAndSendANewOrder) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol.to_string()),
            ("side".to_owned(), request.side.to_string()),
            ("type".to_owned(), request.r#type.to_string()),
            (
                "cancelReplaceMode".to_owned(),
                request.cancel_replace_mode.to_string(),
            ),
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

        if let Some(cancel_new_client_order_id) = request.cancel_new_client_order_id {
            params.push((
                "cancelNewClientOrderId".to_owned(),
                cancel_new_client_order_id,
            ));
        }

        if let Some(cancel_orig_client_order_id) = request.cancel_orig_client_order_id {
            params.push((
                "cancelOrigClientOrderId".to_owned(),
                cancel_orig_client_order_id,
            ));
        }

        if let Some(cancel_order_id) = request.cancel_order_id {
            params.push(("cancelOrderId".to_owned(), cancel_order_id.to_string()));
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
            path: "/api/v3/order/cancelReplace".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CancelAnExistingOrderAndSendANewOrder;
    use crate::http::{request::Request, Credentials, Method};
    use crate::trade::order::{CancelReplaceMode, Side, TimeInForce};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_cancel_an_existing_order_and_send_a_new_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = CancelAnExistingOrderAndSendANewOrder::new(
            "BNBUSDT",
            Side::Sell,
            "LIMIT",
            CancelReplaceMode::StopOnFailure,
        )
        .time_in_force(TimeInForce::Gtc)
        .quantity(dec!(10.1))
        .price(dec!(295.92))
        .cancel_order_id(12)
        .stop_price(dec!(20.01))
        .recv_window(5000)
        .credentials(&credentials)
        .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/order/cancelReplace".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("side".to_owned(), "SELL".to_string()),
                    ("type".to_owned(), "LIMIT".to_string()),
                    (
                        "cancelReplaceMode".to_owned(),
                        "STOP_ON_FAILURE".to_string()
                    ),
                    ("timeInForce".to_owned(), "GTC".to_string()),
                    ("quantity".to_owned(), "10.1".to_string()),
                    ("price".to_owned(), "295.92".to_string()),
                    ("cancelOrderId".to_owned(), "12".to_string()),
                    ("stopPrice".to_owned(), "20.01".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
