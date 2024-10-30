//! Account/Trade
//!
//! [API Documentation]()
pub mod account;
pub mod all_orders;
pub mod cancel_an_existing_order_and_send_a_new_order;
pub mod cancel_oco_order;
pub mod cancel_open_orders;
pub mod cancel_order;
pub mod get_allocations;
pub mod get_commission_rates;
pub mod get_oco_order;
pub mod get_oco_orders;
pub mod get_open_oco_orders;
pub mod get_order;
pub mod get_prevented_matches;
pub mod my_trades;
pub mod new_oco_order;
pub mod new_order;
pub mod new_order_test;
pub mod new_oto_order;
pub mod new_otoco_order;
pub mod open_orders;
pub mod order;
pub mod order_limit_usage;

use rust_decimal::Decimal;

use account::Account;
use all_orders::AllOrders;
use cancel_an_existing_order_and_send_a_new_order::CancelAnExistingOrderAndSendANewOrder;
use cancel_oco_order::CancelOCOOrder;
use cancel_open_orders::CancelOpenOrders;
use cancel_order::CancelOrder;
use get_allocations::GetAllocations;
use get_commission_rates::GetCommissionRates;
use get_oco_order::GetOCOOrder;
use get_oco_orders::GetOCOOrders;
use get_open_oco_orders::GetOpenOCOOrders;
use get_order::GetOrder;
use get_prevented_matches::GetPreventedMatches;
use my_trades::MyTrades;
use new_oco_order::NewOCOOrder;
use new_order::NewOrder;
use new_order_test::NewOrderTest;
use new_oto_order::NewOTOOrder;
use new_otoco_order::NewOTOCOOrder;
use open_orders::OpenOrders;
use order::{CancelReplaceMode, Side, WorkingMandatoryParams};
use order_limit_usage::OrderLimitUsage;

pub fn new_order_test(symbol: &str, side: Side, r#type: &str) -> NewOrderTest {
    NewOrderTest::new(symbol, side, r#type)
}

pub fn get_order(symbol: &str) -> GetOrder {
    GetOrder::new(symbol)
}

pub fn cancel_an_existing_order_and_send_a_new_order(
    symbol: &str,
    side: Side,
    r#type: &str,
    cancel_replace_mode: CancelReplaceMode,
) -> CancelAnExistingOrderAndSendANewOrder {
    CancelAnExistingOrderAndSendANewOrder::new(symbol, side, r#type, cancel_replace_mode)
}

pub fn new_order(symbol: &str, side: Side, r#type: &str) -> NewOrder {
    NewOrder::new(symbol, side, r#type)
}

pub fn cancel_order(symbol: &str) -> CancelOrder {
    CancelOrder::new(symbol)
}

pub fn open_orders() -> OpenOrders {
    OpenOrders::new()
}

pub fn cancel_open_orders(symbol: &str) -> CancelOpenOrders {
    CancelOpenOrders::new(symbol)
}

pub fn all_orders(symbol: &str) -> AllOrders {
    AllOrders::new(symbol)
}

pub fn new_oco_order(
    symbol: &str,
    side: Side,
    quantity: Decimal,
    above_type: &str,
    below_type: &str,
) -> NewOCOOrder {
    NewOCOOrder::new(symbol, side, quantity, above_type, below_type)
}

pub fn new_oto_order(
    symbol: &str,
    working_mandatory_params: WorkingMandatoryParams,
    pending_type: &str,
    pending_side: Side,
    pending_quantity: Decimal,
) -> NewOTOOrder {
    NewOTOOrder::new(
        symbol,
        working_mandatory_params,
        pending_type,
        pending_side,
        pending_quantity,
    )
}

pub fn new_otoco_order(
    symbol: &str,
    working_mandatory_params: WorkingMandatoryParams,
    pending_side: Side,
    pending_quantity: Decimal,
    pending_above_type: &str,
) -> NewOTOCOOrder {
    NewOTOCOOrder::new(
        symbol,
        working_mandatory_params,
        pending_side,
        pending_quantity,
        pending_above_type,
    )
}

pub fn get_allocations(symbol: &str) -> GetAllocations {
    GetAllocations::new(symbol)
}

pub fn get_commission_rates(symbol: &str) -> GetCommissionRates {
    GetCommissionRates::new(symbol)
}

pub fn get_oco_order() -> GetOCOOrder {
    GetOCOOrder::new()
}

pub fn cancel_oco_order(symbol: &str) -> CancelOCOOrder {
    CancelOCOOrder::new(symbol)
}

pub fn get_oco_orders() -> GetOCOOrders {
    GetOCOOrders::new()
}

pub fn get_open_oco_orders() -> GetOpenOCOOrders {
    GetOpenOCOOrders::new()
}

pub fn get_prevented_matches(symbol: &str) -> GetPreventedMatches {
    GetPreventedMatches::new(symbol)
}

pub fn account() -> Account {
    Account::new()
}

pub fn my_trades(symbol: &str) -> MyTrades {
    MyTrades::new(symbol)
}

pub fn order_limit_usage() -> OrderLimitUsage {
    OrderLimitUsage::new()
}
