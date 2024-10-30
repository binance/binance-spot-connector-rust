use crate::http::{request::Request, Credentials, Method};
use crate::trade::order::{NewOrderResponseType, Side, TimeInForce, WorkingMandatoryParams};
use rust_decimal::Decimal;

/// `POST /api/v3/orderList/otoco`
///
/// Place an OTOCO.
///
/// * An OTOCO (One-Triggers-One-Cancels-the-Other) is an order list comprised of 3 orders.
/// * The first order is called the working order and must be `LIMIT` or `LIMIT_MAKER`. Initially, only the working order goes on the order book.
///   - The behavior of the working order is the same as the OTO.
/// * OTOCO has 2 pending orders (pending above and pending below), forming an OCO pair. The pending orders are only placed on the order book when the working order gets fully filled.
///   - The rules of the pending above and pending below follow the same rules as the Order List OCO.
/// * OTOCOs add 3 orders against the unfilled order count, EXCHANGE_MAX_NUM_ORDERS filter, and MAX_NUM_ORDERS filter.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade::{self, order::{Side, TimeInForce, WorkingMandatoryParams}};
/// use rust_decimal_macros::dec;
///
/// let request = trade::new_otoco_order("BNBUSDT", WorkingMandatoryParams::new("LIMIT", Side::Sell, dec!(305), dec!(0.5)), Side::Sell, dec!(0.5), "LIMIT_MAKER").working_time_in_force(TimeInForce::Gtc).pending_above_price(dec!(308)).pending_below_type("STOP_LOSS_LIMIT").pending_below_stop_price(dec!(300.5)).pending_below_trailing_delta(dec!(30)).pending_below_time_in_force(TimeInForce::Gtc).pending_below_price(dec!(301));
/// ```
pub struct NewOTOCOOrder {
    symbol: String,
    working_mandatory_params: WorkingMandatoryParams,
    pending_side: Side,
    pending_quantity: Decimal,
    pending_above_type: String,
    list_client_order_id: Option<String>,
    new_order_resp_type: Option<NewOrderResponseType>,
    self_trade_prevention: Option<String>,
    working_client_order_id: Option<String>,
    working_iceberg_qty: Option<Decimal>,
    working_time_in_force: Option<TimeInForce>,
    working_strategy_id: Option<u64>,
    working_strategy_type: Option<u64>,
    pending_above_client_order_id: Option<String>,
    pending_above_price: Option<Decimal>,
    pending_above_stop_price: Option<Decimal>,
    pending_above_trailing_delta: Option<Decimal>,
    pending_above_iceberg_qty: Option<Decimal>,
    pending_above_time_in_force: Option<TimeInForce>,
    pending_above_strategy_id: Option<u64>,
    pending_above_strategy_type: Option<u64>,
    pending_below_type: Option<String>,
    pending_below_client_order_id: Option<String>,
    pending_below_price: Option<Decimal>,
    pending_below_stop_price: Option<Decimal>,
    pending_below_trailing_delta: Option<Decimal>,
    pending_below_iceberg_qty: Option<Decimal>,
    pending_below_time_in_force: Option<TimeInForce>,
    pending_below_strategy_id: Option<u64>,
    pending_below_strategy_type: Option<u64>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl NewOTOCOOrder {
    pub fn new(
        symbol: &str,
        working_mandatory_params: WorkingMandatoryParams,
        pending_side: Side,
        pending_quantity: Decimal,
        pending_above_type: &str,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            working_mandatory_params,
            pending_side,
            pending_quantity,
            pending_above_type: pending_above_type.to_owned(),
            list_client_order_id: None,
            new_order_resp_type: None,
            self_trade_prevention: None,
            working_client_order_id: None,
            working_iceberg_qty: None,
            working_time_in_force: None,
            working_strategy_id: None,
            working_strategy_type: None,
            pending_above_client_order_id: None,
            pending_above_price: None,
            pending_above_stop_price: None,
            pending_above_trailing_delta: None,
            pending_above_iceberg_qty: None,
            pending_above_time_in_force: None,
            pending_above_strategy_id: None,
            pending_above_strategy_type: None,
            pending_below_type: None,
            pending_below_client_order_id: None,
            pending_below_price: None,
            pending_below_stop_price: None,
            pending_below_trailing_delta: None,
            pending_below_iceberg_qty: None,
            pending_below_time_in_force: None,
            pending_below_strategy_id: None,
            pending_below_strategy_type: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.list_client_order_id = Some(list_client_order_id.to_owned());
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

    pub fn working_client_order_id(mut self, working_client_order_id: &str) -> Self {
        self.working_client_order_id = Some(working_client_order_id.to_owned());
        self
    }

    pub fn working_iceberg_qty(mut self, working_iceberg_qty: Decimal) -> Self {
        self.working_iceberg_qty = Some(working_iceberg_qty);
        self
    }

    pub fn working_time_in_force(mut self, working_time_in_force: TimeInForce) -> Self {
        self.working_time_in_force = Some(working_time_in_force);
        self
    }

    pub fn working_strategy_id(mut self, working_strategy_id: u64) -> Self {
        self.working_strategy_id = Some(working_strategy_id);
        self
    }

    pub fn working_strategy_type(mut self, working_strategy_type: u64) -> Self {
        self.working_strategy_type = Some(working_strategy_type);
        self
    }

    pub fn pending_above_client_order_id(mut self, pending_above_client_order_id: &str) -> Self {
        self.pending_above_client_order_id = Some(pending_above_client_order_id.to_owned());
        self
    }

    pub fn pending_above_price(mut self, pending_above_price: Decimal) -> Self {
        self.pending_above_price = Some(pending_above_price);
        self
    }

    pub fn pending_above_stop_price(mut self, pending_above_stop_price: Decimal) -> Self {
        self.pending_above_stop_price = Some(pending_above_stop_price);
        self
    }

    pub fn pending_above_trailing_delta(mut self, pending_above_trailing_delta: Decimal) -> Self {
        self.pending_above_trailing_delta = Some(pending_above_trailing_delta);
        self
    }

    pub fn pending_above_iceberg_qty(mut self, pending_above_iceberg_qty: Decimal) -> Self {
        self.pending_above_iceberg_qty = Some(pending_above_iceberg_qty);
        self
    }

    pub fn pending_above_time_in_force(mut self, pending_above_time_in_force: TimeInForce) -> Self {
        self.pending_above_time_in_force = Some(pending_above_time_in_force);
        self
    }

    pub fn pending_above_strategy_id(mut self, pending_above_strategy_id: u64) -> Self {
        self.pending_above_strategy_id = Some(pending_above_strategy_id);
        self
    }

    pub fn pending_above_strategy_type(mut self, pending_above_strategy_type: u64) -> Self {
        self.pending_above_strategy_type = Some(pending_above_strategy_type);
        self
    }

    pub fn pending_below_type(mut self, pending_below_type: &str) -> Self {
        self.pending_below_type = Some(pending_below_type.to_owned());
        self
    }

    pub fn pending_below_client_order_id(mut self, pending_below_client_order_id: &str) -> Self {
        self.pending_below_client_order_id = Some(pending_below_client_order_id.to_owned());
        self
    }

    pub fn pending_below_price(mut self, pending_below_price: Decimal) -> Self {
        self.pending_below_price = Some(pending_below_price);
        self
    }

    pub fn pending_below_stop_price(mut self, pending_below_stop_price: Decimal) -> Self {
        self.pending_below_stop_price = Some(pending_below_stop_price);
        self
    }

    pub fn pending_below_trailing_delta(mut self, pending_below_trailing_delta: Decimal) -> Self {
        self.pending_below_trailing_delta = Some(pending_below_trailing_delta);
        self
    }

    pub fn pending_below_iceberg_qty(mut self, pending_below_iceberg_qty: Decimal) -> Self {
        self.pending_below_iceberg_qty = Some(pending_below_iceberg_qty);
        self
    }

    pub fn pending_below_time_in_force(mut self, pending_below_time_in_force: TimeInForce) -> Self {
        self.pending_below_time_in_force = Some(pending_below_time_in_force);
        self
    }

    pub fn pending_below_strategy_id(mut self, pending_below_strategy_id: u64) -> Self {
        self.pending_below_strategy_id = Some(pending_below_strategy_id);
        self
    }

    pub fn pending_below_strategy_type(mut self, pending_below_strategy_type: u64) -> Self {
        self.pending_below_strategy_type = Some(pending_below_strategy_type);
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

impl From<NewOTOCOOrder> for Request {
    fn from(request: NewOTOCOOrder) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol.to_string()),
            (
                "workingType".to_owned(),
                request.working_mandatory_params.working_type.to_string(),
            ),
            (
                "workingSide".to_owned(),
                request.working_mandatory_params.working_side.to_string(),
            ),
            (
                "workingPrice".to_owned(),
                request.working_mandatory_params.working_price.to_string(),
            ),
            (
                "workingQuantity".to_owned(),
                request
                    .working_mandatory_params
                    .working_quantity
                    .to_string(),
            ),
            ("pendingSide".to_owned(), request.pending_side.to_string()),
            (
                "pendingQuantity".to_owned(),
                request.pending_quantity.to_string(),
            ),
            (
                "pendingAboveType".to_owned(),
                request.pending_above_type.to_string(),
            ),
        ];

        if let Some(list_client_order_id) = request.list_client_order_id {
            params.push(("listClientOrderId".to_owned(), list_client_order_id));
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

        if let Some(working_client_order_id) = request.working_client_order_id {
            params.push(("workingClientOrderId".to_owned(), working_client_order_id));
        }

        if let Some(working_iceberg_qty) = request.working_iceberg_qty {
            params.push((
                "workingIcebergQty".to_owned(),
                working_iceberg_qty.to_string(),
            ));
        }

        if let Some(working_time_in_force) = request.working_time_in_force {
            params.push((
                "workingTimeInForce".to_owned(),
                working_time_in_force.to_string(),
            ));
        }

        if let Some(working_strategy_id) = request.working_strategy_id {
            params.push((
                "workingStrategyId".to_owned(),
                working_strategy_id.to_string(),
            ));
        }

        if let Some(working_strategy_type) = request.working_strategy_type {
            params.push((
                "workingStrategyType".to_owned(),
                working_strategy_type.to_string(),
            ));
        }

        if let Some(pending_above_client_order_id) = request.pending_above_client_order_id {
            params.push((
                "pendingAboveClientOrderId".to_owned(),
                pending_above_client_order_id,
            ));
        }

        if let Some(pending_above_price) = request.pending_above_price {
            params.push((
                "pendingAbovePrice".to_owned(),
                pending_above_price.to_string(),
            ));
        }

        if let Some(pending_above_stop_price) = request.pending_above_stop_price {
            params.push((
                "pendingAboveStopPrice".to_owned(),
                pending_above_stop_price.to_string(),
            ));
        }

        if let Some(pending_above_trailing_delta) = request.pending_above_trailing_delta {
            params.push((
                "pendingAboveTrailingDelta".to_owned(),
                pending_above_trailing_delta.to_string(),
            ));
        }

        if let Some(pending_above_iceberg_qty) = request.pending_above_iceberg_qty {
            params.push((
                "pendingAboveIcebergQty".to_owned(),
                pending_above_iceberg_qty.to_string(),
            ));
        }

        if let Some(pending_above_time_in_force) = request.pending_above_time_in_force {
            params.push((
                "pendingAboveTimeInForce".to_owned(),
                pending_above_time_in_force.to_string(),
            ));
        }

        if let Some(pending_above_strategy_id) = request.pending_above_strategy_id {
            params.push((
                "pendingAboveStrategyId".to_owned(),
                pending_above_strategy_id.to_string(),
            ));
        }

        if let Some(pending_above_strategy_type) = request.pending_above_strategy_type {
            params.push((
                "pendingAboveStrategyType".to_owned(),
                pending_above_strategy_type.to_string(),
            ));
        }

        if let Some(pending_below_type) = request.pending_below_type {
            params.push(("pendingBelowType".to_owned(), pending_below_type));
        }

        if let Some(pending_below_client_order_id) = request.pending_below_client_order_id {
            params.push((
                "pendingBelowClientOrderId".to_owned(),
                pending_below_client_order_id,
            ));
        }

        if let Some(pending_below_price) = request.pending_below_price {
            params.push((
                "pendingBelowPrice".to_owned(),
                pending_below_price.to_string(),
            ));
        }

        if let Some(pending_below_stop_price) = request.pending_below_stop_price {
            params.push((
                "pendingBelowStopPrice".to_owned(),
                pending_below_stop_price.to_string(),
            ));
        }

        if let Some(pending_below_trailing_delta) = request.pending_below_trailing_delta {
            params.push((
                "pendingBelowTrailingDelta".to_owned(),
                pending_below_trailing_delta.to_string(),
            ));
        }

        if let Some(pending_below_iceberg_qty) = request.pending_below_iceberg_qty {
            params.push((
                "pendingBelowIcebergQty".to_owned(),
                pending_below_iceberg_qty.to_string(),
            ));
        }

        if let Some(pending_below_time_in_force) = request.pending_below_time_in_force {
            params.push((
                "pendingBelowTimeInForce".to_owned(),
                pending_below_time_in_force.to_string(),
            ));
        }

        if let Some(pending_below_strategy_id) = request.pending_below_strategy_id {
            params.push((
                "pendingBelowStrategyId".to_owned(),
                pending_below_strategy_id.to_string(),
            ));
        }

        if let Some(pending_below_strategy_type) = request.pending_below_strategy_type {
            params.push((
                "pendingBelowStrategyType".to_owned(),
                pending_below_strategy_type.to_string(),
            ));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/orderList/otoco".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewOTOCOOrder;
    use crate::http::{request::Request, Credentials, Method};
    use crate::trade::order::{Side, TimeInForce, WorkingMandatoryParams};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_new_otoco_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = NewOTOCOOrder::new(
            "BNBUSDT",
            WorkingMandatoryParams::new("LIMIT", Side::Sell, dec!(305), dec!(0.5)),
            Side::Sell,
            dec!(0.5),
            "LIMIT_MAKER",
        )
        .working_time_in_force(TimeInForce::Gtc)
        .pending_above_price(dec!(308))
        .pending_below_type("STOP_LOSS_LIMIT")
        .pending_below_stop_price(dec!(300.5))
        .pending_below_trailing_delta(dec!(30))
        .pending_below_time_in_force(TimeInForce::Gtc)
        .pending_below_price(dec!(301))
        .credentials(&credentials)
        .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/orderList/otoco".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("workingType".to_owned(), "LIMIT".to_string()),
                    ("workingSide".to_owned(), "SELL".to_string()),
                    ("workingPrice".to_owned(), "305".to_string()),
                    ("workingQuantity".to_owned(), "0.5".to_string()),
                    ("pendingSide".to_owned(), "SELL".to_string()),
                    ("pendingQuantity".to_owned(), "0.5".to_string()),
                    ("pendingAboveType".to_owned(), "LIMIT_MAKER".to_string()),
                    ("workingTimeInForce".to_owned(), "GTC".to_string()),
                    ("pendingAbovePrice".to_owned(), "308".to_string()),
                    ("pendingBelowType".to_owned(), "STOP_LOSS_LIMIT".to_string()),
                    ("pendingBelowPrice".to_owned(), "301".to_string()),
                    ("pendingBelowStopPrice".to_owned(), "300.5".to_string()),
                    ("pendingBelowTrailingDelta".to_owned(), "30".to_string()),
                    ("pendingBelowTimeInForce".to_owned(), "GTC".to_string()),
                ],
                sign: true
            }
        );
    }
}
