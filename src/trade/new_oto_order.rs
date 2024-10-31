use crate::http::{request::Request, Credentials, Method};
use crate::trade::order::{NewOrderResponseType, Side, TimeInForce, WorkingMandatoryParams};
use rust_decimal::Decimal;

/// `POST /api/v3/orderList/oto`
///
/// Places an OTO.
///
/// * An OTO (One-Triggers-the-Other) is an order list comprised of 2 orders.
/// * The first order is called the working order and must be `LIMIT` or `LIMIT_MAKER`. Initially, only the working order goes on the order book.
/// * The second order is called the pending order. It can be any order type except for `MARKET` orders using parameter `quoteOrderQty`. The pending order is only placed on the order book when the working order gets fully filled.
/// * If either the working order or the pending order is cancelled individually, the other order in the order list will also be canceled or expired.
/// * When the order list is placed, if the working order gets immediately fully filled, the placement response will show the working order as `FILLED` but the pending order will still appear as `PENDING_NEW`. You need to query the status of the pending order again to see its updated status.
/// * OTOs add 2 orders to the unfilled order count, `EXCHANGE_MAX_NUM_ORDERS` filter and `MAX_NUM_ORDERS` filter.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::trade::{self, order::{Side, TimeInForce, WorkingMandatoryParams}};
/// use rust_decimal_macros::dec;
///
/// let request = trade::new_oto_order("BNBUSDT", WorkingMandatoryParams::new("LIMIT", Side::Buy, dec!(596.0), dec!(1.0)), "LIMIT_MAKER", Side::Buy, dec!(1.0)).working_time_in_force(TimeInForce::Gtc).pending_price(dec!(598.1));
/// ```
pub struct NewOTOOrder {
    symbol: String,
    working_mandatory_params: WorkingMandatoryParams,
    pending_type: String,
    pending_side: Side,
    pending_quantity: Decimal,
    list_client_order_id: Option<String>,
    new_order_resp_type: Option<NewOrderResponseType>,
    self_trade_prevention: Option<String>,
    working_client_order_id: Option<String>,
    working_iceberg_qty: Option<Decimal>,
    working_time_in_force: Option<TimeInForce>,
    working_strategy_id: Option<u64>,
    working_strategy_type: Option<u64>,
    pending_client_order_id: Option<String>,
    pending_price: Option<Decimal>,
    pending_stop_price: Option<Decimal>,
    pending_trailing_delta: Option<Decimal>,
    pending_iceberg_qty: Option<Decimal>,
    pending_time_in_force: Option<TimeInForce>,
    pending_strategy_id: Option<u64>,
    pending_strategy_type: Option<u64>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl NewOTOOrder {
    pub fn new(
        symbol: &str,
        working_mandatory_params: WorkingMandatoryParams,
        pending_type: &str,
        pending_side: Side,
        pending_quantity: Decimal,
    ) -> Self {
        Self {
            symbol: symbol.to_owned(),
            working_mandatory_params,
            pending_type: pending_type.to_owned(),
            pending_side,
            pending_quantity,
            list_client_order_id: None,
            new_order_resp_type: None,
            self_trade_prevention: None,
            working_client_order_id: None,
            working_iceberg_qty: None,
            working_time_in_force: None,
            working_strategy_id: None,
            working_strategy_type: None,
            pending_client_order_id: None,
            pending_price: None,
            pending_stop_price: None,
            pending_trailing_delta: None,
            pending_iceberg_qty: None,
            pending_time_in_force: None,
            pending_strategy_id: None,
            pending_strategy_type: None,
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

    pub fn pending_client_order_id(mut self, pending_client_order_id: &str) -> Self {
        self.pending_client_order_id = Some(pending_client_order_id.to_owned());
        self
    }

    pub fn pending_price(mut self, pending_price: Decimal) -> Self {
        self.pending_price = Some(pending_price);
        self
    }

    pub fn pending_stop_price(mut self, pending_stop_price: Decimal) -> Self {
        self.pending_stop_price = Some(pending_stop_price);
        self
    }

    pub fn pending_trailing_delta(mut self, pending_trailing_delta: Decimal) -> Self {
        self.pending_trailing_delta = Some(pending_trailing_delta);
        self
    }

    pub fn pending_iceberg_qty(mut self, pending_iceberg_qty: Decimal) -> Self {
        self.pending_iceberg_qty = Some(pending_iceberg_qty);
        self
    }

    pub fn pending_time_in_force(mut self, pending_time_in_force: TimeInForce) -> Self {
        self.pending_time_in_force = Some(pending_time_in_force);
        self
    }

    pub fn pending_strategy_id(mut self, pending_strategy_id: u64) -> Self {
        self.pending_strategy_id = Some(pending_strategy_id);
        self
    }

    pub fn pending_strategy_type(mut self, pending_strategy_type: u64) -> Self {
        self.pending_strategy_type = Some(pending_strategy_type);
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

impl From<NewOTOOrder> for Request {
    fn from(request: NewOTOOrder) -> Request {
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
            ("pendingType".to_owned(), request.pending_type.to_string()),
            ("pendingSide".to_owned(), request.pending_side.to_string()),
            (
                "pendingQuantity".to_owned(),
                request.pending_quantity.to_string(),
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

        if let Some(pending_client_order_id) = request.pending_client_order_id {
            params.push(("pendingClientOrderId".to_owned(), pending_client_order_id));
        }

        if let Some(pending_price) = request.pending_price {
            params.push(("pendingPrice".to_owned(), pending_price.to_string()));
        }

        if let Some(pending_stop_price) = request.pending_stop_price {
            params.push((
                "pendingStopPrice".to_owned(),
                pending_stop_price.to_string(),
            ));
        }

        if let Some(pending_trailing_delta) = request.pending_trailing_delta {
            params.push((
                "pendingTrailingDelta".to_owned(),
                pending_trailing_delta.to_string(),
            ));
        }

        if let Some(pending_iceberg_qty) = request.pending_iceberg_qty {
            params.push((
                "pendingIcebergQty".to_owned(),
                pending_iceberg_qty.to_string(),
            ));
        }

        if let Some(pending_time_in_force) = request.pending_time_in_force {
            params.push((
                "pendingTimeInForce".to_owned(),
                pending_time_in_force.to_string(),
            ));
        }

        if let Some(pending_strategy_id) = request.pending_strategy_id {
            params.push((
                "pendingStrategyId".to_owned(),
                pending_strategy_id.to_string(),
            ));
        }

        if let Some(pending_strategy_type) = request.pending_strategy_type {
            params.push((
                "pendingStrategyType".to_owned(),
                pending_strategy_type.to_string(),
            ));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/api/v3/orderList/oto".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewOTOOrder;
    use crate::http::{request::Request, Credentials, Method};
    use crate::trade::order::{Side, TimeInForce, WorkingMandatoryParams};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn trade_new_oto_order_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = NewOTOOrder::new(
            "BNBUSDT",
            WorkingMandatoryParams::new("LIMIT", Side::Buy, dec!(596.0), dec!(1.0)),
            "LIMIT_MAKER",
            Side::Buy,
            dec!(1.0),
        )
        .working_time_in_force(TimeInForce::Gtc)
        .pending_price(dec!(598.1))
        .credentials(&credentials)
        .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/orderList/oto".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("symbol".to_owned(), "BNBUSDT".to_string()),
                    ("workingType".to_owned(), "LIMIT".to_string()),
                    ("workingSide".to_owned(), "BUY".to_string()),
                    ("workingPrice".to_owned(), "596.0".to_string()),
                    ("workingQuantity".to_owned(), "1.0".to_string()),
                    ("pendingType".to_owned(), "LIMIT_MAKER".to_string()),
                    ("pendingSide".to_owned(), "BUY".to_string()),
                    ("pendingQuantity".to_owned(), "1.0".to_string()),
                    ("workingTimeInForce".to_owned(), "GTC".to_string()),
                    ("pendingPrice".to_owned(), "598.1".to_string()),
                ],
                sign: true
            }
        );
    }
}
