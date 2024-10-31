use crate::http::{request::Request, Credentials, Method};
use crate::trade::order::{NewOrderResponseType, Side, TimeInForce};
use rust_decimal::Decimal;

/// `POST /api/v3/orderList/oco`
///
/// Send in an one-cancels-the-other (OCO) pair, where activation of one order immediately cancels the other.
///
/// * An OCO has 2 orders called the above order and below order.
/// * One of the orders must be a `LIMIT_MAKER` order and the other must be `STOP_LOSS` or `STOP_LOSS_LIMIT` order.
/// * Price restrictions:
///   - If the OCO is on the `SELL` side: `LIMIT_MAKER`, `price` > Last Traded Price > `stopPrice`
///   - If the OCO is on the `BUY` side: `LIMIT_MAKER`, `price` < Last Traded Price < `stopPrice`
/// * OCOs add 2 orders to the unfilled order count, `EXCHANGE_MAX_ORDERS` filter, and the `MAX_NUM_ORDERS` filter.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade::{self, order::{Side, TimeInForce}};
/// use rust_decimal_macros::dec;
///
/// let request = trade::new_oco_order("BNBUSDT", Side::Sell, dec!(1.0), "LIMIT_MAKER", "STOP_LOSS_LIMIT").above_price(dec!(595.1)).below_price(dec!(585.3)).below_stop_price(dec!(583.2)).below_trailing_delta(dec!(60.0)).below_time_in_force(TimeInForce::Gtc);
/// ```
pub struct NewOCOOrder {
    symbol: String,
    side: Side,
    quantity: Decimal,
    above_type: String,
    below_type: String,
    list_client_order_id: Option<String>,
    above_client_order_id: Option<String>,
    above_iceberg_qty: Option<Decimal>,
    above_price: Option<Decimal>,
    above_stop_price: Option<Decimal>,
    above_trailing_delta: Option<Decimal>,
    above_time_in_force: Option<TimeInForce>,
    above_strategy_id: Option<u64>,
    above_strategy_type: Option<u64>,
    below_client_order_id: Option<String>,
    below_iceberg_qty: Option<Decimal>,
    below_price: Option<Decimal>,
    below_stop_price: Option<Decimal>,
    below_trailing_delta: Option<Decimal>,
    below_time_in_force: Option<TimeInForce>,
    below_strategy_id: Option<u64>,
    below_strategy_type: Option<u64>,
    new_order_resp_type: Option<NewOrderResponseType>,
    self_trade_prevention: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl NewOCOOrder {
    pub fn new(
        symbol: &str,
        side: Side,
        quantity: Decimal,
        above_type: &str,
        below_type: &str,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            quantity,
            above_type: above_type.to_owned(),
            below_type: below_type.to_owned(),
            list_client_order_id: None,
            above_client_order_id: None,
            above_iceberg_qty: None,
            above_price: None,
            above_stop_price: None,
            above_trailing_delta: None,
            above_time_in_force: None,
            above_strategy_id: None,
            above_strategy_type: None,
            below_client_order_id: None,
            below_iceberg_qty: None,
            below_price: None,
            below_stop_price: None,
            below_trailing_delta: None,
            below_time_in_force: None,
            below_strategy_id: None,
            below_strategy_type: None,
            new_order_resp_type: None,
            self_trade_prevention: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
        self
    }

    pub fn above_client_order_id(mut self, above_client_order_id: &str) -> Self {
        self.above_client_order_id = Some(above_client_order_id.to_owned());
        self
    }

    pub fn above_iceberg_qty(mut self, above_iceberg_qty: Decimal) -> Self {
        self.above_iceberg_qty = Some(above_iceberg_qty);
        self
    }

    pub fn above_price(mut self, above_price: Decimal) -> Self {
        self.above_price = Some(above_price);
        self
    }

    pub fn above_stop_price(mut self, above_stop_price: Decimal) -> Self {
        self.above_stop_price = Some(above_stop_price);
        self
    }

    pub fn above_trailing_delta(mut self, above_trailing_delta: Decimal) -> Self {
        self.above_trailing_delta = Some(above_trailing_delta);
        self
    }

    pub fn above_time_in_force(mut self, above_time_in_force: TimeInForce) -> Self {
        self.above_time_in_force = Some(above_time_in_force);
        self
    }

    pub fn above_strategy_id(mut self, above_strategy_id: u64) -> Self {
        self.above_strategy_id = Some(above_strategy_id);
        self
    }

    pub fn above_strategy_type(mut self, above_strategy_type: u64) -> Self {
        self.above_strategy_type = Some(above_strategy_type);
        self
    }

    pub fn below_client_order_id(mut self, below_client_order_id: &str) -> Self {
        self.below_client_order_id = Some(below_client_order_id.to_owned());
        self
    }

    pub fn below_iceberg_qty(mut self, below_iceberg_qty: Decimal) -> Self {
        self.below_iceberg_qty = Some(below_iceberg_qty);
        self
    }

    pub fn below_price(mut self, below_price: Decimal) -> Self {
        self.below_price = Some(below_price);
        self
    }

    pub fn below_stop_price(mut self, below_stop_price: Decimal) -> Self {
        self.below_stop_price = Some(below_stop_price);
        self
    }

    pub fn below_trailing_delta(mut self, below_trailing_delta: Decimal) -> Self {
        self.below_trailing_delta = Some(below_trailing_delta);
        self
    }

    pub fn below_time_in_force(mut self, below_time_in_force: TimeInForce) -> Self {
        self.below_time_in_force = Some(below_time_in_force);
        self
    }

    pub fn below_strategy_id(mut self, below_strategy_id: u64) -> Self {
        self.below_strategy_id = Some(below_strategy_id);
        self
    }

    pub fn below_strategy_type(mut self, below_strategy_type: u64) -> Self {
        self.below_strategy_type = Some(below_strategy_type);
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: NewOrderResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn self_trade_prevention(mut self, self_trade_prevention: &str) -> Self {
        self.self_trade_prevention = Some(self_trade_prevention.to_owned());
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

impl From<NewOCOOrder> for Request {
    fn from(request: NewOCOOrder) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol.to_string()),
            ("side".to_owned(), request.side.to_string()),
            ("quantity".to_owned(), request.quantity.to_string()),
            ("aboveType".to_owned(), request.above_type.to_string()),
            ("belowType".to_owned(), request.below_type.to_string()),
        ];

        if let Some(list_client_order_id) = request.list_client_order_id {
            params.push(("listClientOrderId".to_owned(), list_client_order_id));
        }

        if let Some(above_client_order_id) = request.above_client_order_id {
            params.push(("aboveClientOrderId".to_owned(), above_client_order_id));
        }

        if let Some(above_iceberg_qty) = request.above_iceberg_qty {
            params.push(("aboveIcebergQty".to_owned(), above_iceberg_qty.to_string()));
        }

        if let Some(above_price) = request.above_price {
            params.push(("abovePrice".to_owned(), above_price.to_string()));
        }

        if let Some(above_stop_price) = request.above_stop_price {
            params.push(("aboveStopPrice".to_owned(), above_stop_price.to_string()));
        }

        if let Some(above_trailing_delta) = request.above_trailing_delta {
            params.push((
                "aboveTrailingDelta".to_owned(),
                above_trailing_delta.to_string(),
            ));
        }

        if let Some(above_time_in_force) = request.above_time_in_force {
            params.push((
                "aboveTimeInForce".to_owned(),
                above_time_in_force.to_string(),
            ));
        }

        if let Some(above_strategy_id) = request.above_strategy_id {
            params.push(("aboveStrategyId".to_owned(), above_strategy_id.to_string()));
        }

        if let Some(above_strategy_type) = request.above_strategy_type {
            params.push((
                "aboveStrategyType".to_owned(),
                above_strategy_type.to_string(),
            ));
        }

        if let Some(below_client_order_id) = request.below_client_order_id {
            params.push(("belowClientOrderId".to_owned(), below_client_order_id));
        }

        if let Some(below_iceberg_qty) = request.below_iceberg_qty {
            params.push(("belowIcebergQty".to_owned(), below_iceberg_qty.to_string()));
        }

        if let Some(below_price) = request.below_price {
            params.push(("belowPrice".to_owned(), below_price.to_string()));
        }

        if let Some(below_stop_price) = request.below_stop_price {
            params.push(("belowStopPrice".to_owned(), below_stop_price.to_string()));
        }

        if let Some(below_trailing_delta) = request.below_trailing_delta {
            params.push((
                "belowTrailingDelta".to_owned(),
                below_trailing_delta.to_string(),
            ));
        }

        if let Some(below_time_in_force) = request.below_time_in_force {
            params.push((
                "belowTimeInForce".to_owned(),
                below_time_in_force.to_string(),
            ));
        }

        if let Some(below_strategy_id) = request.below_strategy_id {
            params.push(("belowStrategyId".to_owned(), below_strategy_id.to_string()));
        }

        if let Some(below_strategy_type) = request.below_strategy_type {
            params.push((
                "belowStrategyType".to_owned(),
                below_strategy_type.to_string(),
            ));
        }

        if let Some(new_order_resp_type) = request.new_order_resp_type {
            params.push((
                "newOrderRespType".to_owned(),
                new_order_resp_type.to_string(),
            ));
        }

        if let Some(self_trade_prevention) = request.self_trade_prevention {
            params.push(("selfTradePrevention".to_owned(), self_trade_prevention));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/orderList/oco".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewOCOOrder;
    use crate::http::{request::Request, Credentials, Method};
    use crate::trade::order::{Side, TimeInForce};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_new_oco_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = NewOCOOrder::new(
            "BNBUSDT",
            Side::Sell,
            dec!(1.0),
            "LIMIT_MAKER",
            "STOP_LOSS_LIMIT",
        )
        .above_price(dec!(595.1))
        .below_price(dec!(585.3))
        .below_stop_price(dec!(583.2))
        .below_trailing_delta(dec!(60.0))
        .below_time_in_force(TimeInForce::Gtc)
        .credentials(&credentials)
        .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/orderList/oco".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("side".to_owned(), "SELL".to_string()),
                    ("quantity".to_owned(), "1.0".to_string()),
                    ("aboveType".to_owned(), "LIMIT_MAKER".to_string()),
                    ("belowType".to_owned(), "STOP_LOSS_LIMIT".to_string()),
                    ("abovePrice".to_owned(), "595.1".to_string()),
                    ("belowPrice".to_owned(), "585.3".to_string()),
                    ("belowStopPrice".to_owned(), "583.2".to_string()),
                    ("belowTrailingDelta".to_owned(), "60.0".to_string()),
                    ("belowTimeInForce".to_owned(), "GTC".to_string()),
                ],
                sign: true
            }
        );
    }
}
