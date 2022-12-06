//! Market Data

pub mod bnb_burn_status;
pub mod isolated_margin_account;
pub mod isolated_margin_account_limit;
pub mod isolated_margin_all_symbols;
pub mod isolated_margin_disable_account;
pub mod isolated_margin_enable_account;
pub mod isolated_margin_fee_data;
pub mod isolated_margin_symbol;
pub mod isolated_margin_tier_data;
pub mod isolated_margin_transfer;
pub mod isolated_margin_transfer_history;
pub mod margin_account;
pub mod margin_all_assets;
pub mod margin_all_oco_order;
pub mod margin_all_orders;
pub mod margin_all_pairs;
pub mod margin_asset;
pub mod margin_borrow;
pub mod margin_cancel_oco_order;
pub mod margin_cancel_open_orders;
pub mod margin_cancel_order;
pub mod margin_dustlog;
pub mod margin_fee_data;
pub mod margin_force_liquidation_record;
pub mod margin_interest_history;
pub mod margin_interest_rate_history;
pub mod margin_loan_record;
pub mod margin_max_borrowable;
pub mod margin_max_transferable;
pub mod margin_my_trades;
pub mod margin_new_oco_order;
pub mod margin_new_order;
pub mod margin_oco_order;
pub mod margin_open_oco_order;
pub mod margin_open_orders;
pub mod margin_order;
pub mod margin_order_count_usage;
pub mod margin_pair;
pub mod margin_price_index;
pub mod margin_repay;
pub mod margin_repay_record;
pub mod margin_transfer;
pub mod margin_transfer_history;
pub mod toggle_bnb_burn;

use rust_decimal::Decimal;

use bnb_burn_status::BNBBurnStatus;
use isolated_margin_account::IsolatedMarginAccount;
use isolated_margin_account_limit::IsolatedMarginAccountLimit;
use isolated_margin_all_symbols::IsolatedMarginAllSymbols;
use isolated_margin_disable_account::IsolatedMarginDisableAccount;
use isolated_margin_enable_account::IsolatedMarginEnableAccount;
use isolated_margin_fee_data::IsolatedMarginFeeData;
use isolated_margin_symbol::IsolatedMarginSymbol;
use isolated_margin_tier_data::IsolatedMarginTierData;
use isolated_margin_transfer::IsolatedMarginTransfer;
use isolated_margin_transfer_history::IsolatedMarginTransferHistory;
use margin_account::MarginAccount;
use margin_all_assets::MarginAllAssets;
use margin_all_oco_order::MarginAllOCOOrder;
use margin_all_orders::MarginAllOrders;
use margin_all_pairs::MarginAllPairs;
use margin_asset::MarginAsset;
use margin_borrow::MarginBorrow;
use margin_cancel_oco_order::MarginCancelOCOOrder;
use margin_cancel_open_orders::MarginCancelOpenOrders;
use margin_cancel_order::MarginCancelOrder;
use margin_dustlog::MarginDustlog;
use margin_fee_data::MarginFeeData;
use margin_force_liquidation_record::MarginForceLiquidationRecord;
use margin_interest_history::MarginInterestHistory;
use margin_interest_rate_history::MarginInterestRateHistory;
use margin_loan_record::MarginLoanRecord;
use margin_max_borrowable::MarginMaxBorrowable;
use margin_max_transferable::MarginMaxTransferable;
use margin_my_trades::MarginMyTrades;
use margin_new_oco_order::MarginNewOCOOrder;
use margin_new_order::MarginNewOrder;
use margin_oco_order::MarginOCOOrder;
use margin_open_oco_order::MarginOpenOCOOrder;
use margin_open_orders::MarginOpenOrders;
use margin_order::MarginOrder;
use margin_order_count_usage::MarginOrderCountUsage;
use margin_pair::MarginPair;
use margin_price_index::MarginPriceIndex;
use margin_repay::MarginRepay;
use margin_repay_record::MarginRepayRecord;
use margin_transfer::MarginTransfer;
use margin_transfer_history::MarginTransferHistory;
use toggle_bnb_burn::ToggleBNBBurn;

pub fn margin_transfer(asset: &str, amount: Decimal, r#type: u32) -> MarginTransfer {
    MarginTransfer::new(asset, amount, r#type)
}

pub fn margin_transfer_history() -> MarginTransferHistory {
    MarginTransferHistory::new()
}

pub fn margin_borrow(asset: &str, amount: Decimal) -> MarginBorrow {
    MarginBorrow::new(asset, amount)
}

pub fn margin_loan_record(asset: &str) -> MarginLoanRecord {
    MarginLoanRecord::new(asset)
}

pub fn margin_repay(asset: &str, amount: Decimal) -> MarginRepay {
    MarginRepay::new(asset, amount)
}

pub fn margin_repay_record(asset: &str) -> MarginRepayRecord {
    MarginRepayRecord::new(asset)
}

pub fn margin_asset(asset: &str) -> MarginAsset {
    MarginAsset::new(asset)
}

pub fn margin_pair(symbol: &str) -> MarginPair {
    MarginPair::new(symbol)
}

pub fn margin_all_assets() -> MarginAllAssets {
    MarginAllAssets::new()
}

pub fn margin_all_pairs() -> MarginAllPairs {
    MarginAllPairs::new()
}

pub fn margin_price_index(symbol: &str) -> MarginPriceIndex {
    MarginPriceIndex::new(symbol)
}

pub fn margin_order(symbol: &str) -> MarginOrder {
    MarginOrder::new(symbol)
}

pub fn margin_new_order(symbol: &str, side: &str, r#type: &str) -> MarginNewOrder {
    MarginNewOrder::new(symbol, side, r#type)
}

pub fn margin_cancel_order(symbol: &str) -> MarginCancelOrder {
    MarginCancelOrder::new(symbol)
}

pub fn margin_interest_history() -> MarginInterestHistory {
    MarginInterestHistory::new()
}

pub fn margin_force_liquidation_record() -> MarginForceLiquidationRecord {
    MarginForceLiquidationRecord::new()
}

pub fn margin_account() -> MarginAccount {
    MarginAccount::new()
}

pub fn margin_open_orders() -> MarginOpenOrders {
    MarginOpenOrders::new()
}

pub fn margin_cancel_open_orders(symbol: &str) -> MarginCancelOpenOrders {
    MarginCancelOpenOrders::new(symbol)
}

pub fn margin_all_orders(symbol: &str) -> MarginAllOrders {
    MarginAllOrders::new(symbol)
}

pub fn margin_new_oco_order(
    symbol: &str,
    side: &str,
    quantity: Decimal,
    price: Decimal,
    stop_price: Decimal,
) -> MarginNewOCOOrder {
    MarginNewOCOOrder::new(symbol, side, quantity, price, stop_price)
}

pub fn margin_oco_order() -> MarginOCOOrder {
    MarginOCOOrder::new()
}

pub fn margin_cancel_oco_order(symbol: &str) -> MarginCancelOCOOrder {
    MarginCancelOCOOrder::new(symbol)
}

pub fn margin_all_oco_order() -> MarginAllOCOOrder {
    MarginAllOCOOrder::new()
}

pub fn margin_open_oco_order() -> MarginOpenOCOOrder {
    MarginOpenOCOOrder::new()
}

pub fn margin_my_trades(symbol: &str) -> MarginMyTrades {
    MarginMyTrades::new(symbol)
}

pub fn margin_max_borrowable(asset: &str) -> MarginMaxBorrowable {
    MarginMaxBorrowable::new(asset)
}

pub fn margin_max_transferable(asset: &str) -> MarginMaxTransferable {
    MarginMaxTransferable::new(asset)
}

pub fn isolated_margin_transfer_history(symbol: &str) -> IsolatedMarginTransferHistory {
    IsolatedMarginTransferHistory::new(symbol)
}

pub fn isolated_margin_transfer(
    asset: &str,
    symbol: &str,
    trans_from: &str,
    trans_to: &str,
    amount: Decimal,
) -> IsolatedMarginTransfer {
    IsolatedMarginTransfer::new(asset, symbol, trans_from, trans_to, amount)
}

pub fn isolated_margin_account() -> IsolatedMarginAccount {
    IsolatedMarginAccount::new()
}

pub fn isolated_margin_disable_account(symbol: &str) -> IsolatedMarginDisableAccount {
    IsolatedMarginDisableAccount::new(symbol)
}

pub fn isolated_margin_enable_account(symbol: &str) -> IsolatedMarginEnableAccount {
    IsolatedMarginEnableAccount::new(symbol)
}

pub fn isolated_margin_account_limit() -> IsolatedMarginAccountLimit {
    IsolatedMarginAccountLimit::new()
}

pub fn isolated_margin_symbol(symbol: &str) -> IsolatedMarginSymbol {
    IsolatedMarginSymbol::new(symbol)
}

pub fn isolated_margin_all_symbols() -> IsolatedMarginAllSymbols {
    IsolatedMarginAllSymbols::new()
}

pub fn toggle_bnb_burn() -> ToggleBNBBurn {
    ToggleBNBBurn::new()
}

pub fn bnb_burn_status() -> BNBBurnStatus {
    BNBBurnStatus::new()
}

pub fn margin_interest_rate_history(asset: &str) -> MarginInterestRateHistory {
    MarginInterestRateHistory::new(asset)
}

pub fn margin_fee_data() -> MarginFeeData {
    MarginFeeData::new()
}

pub fn isolated_margin_fee_data() -> IsolatedMarginFeeData {
    IsolatedMarginFeeData::new()
}

pub fn isolated_margin_tier_data(symbol: &str) -> IsolatedMarginTierData {
    IsolatedMarginTierData::new(symbol)
}

pub fn margin_order_count_usage() -> MarginOrderCountUsage {
    MarginOrderCountUsage::new()
}

pub fn margin_dustlog() -> MarginDustlog {
    MarginDustlog::new()
}
