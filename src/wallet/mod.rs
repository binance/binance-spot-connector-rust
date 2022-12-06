//! Market Data

pub mod account_snapshot;
pub mod account_status;
pub mod api_key_permission;
pub mod api_trading_status;
pub mod asset_detail;
pub mod asset_dividend_record;
pub mod coin_info;
pub mod deposit_address;
pub mod deposit_history;
pub mod disable_fast_withdraw;
pub mod dust_log;
pub mod dust_transfer;
pub mod dustable_assets;
pub mod enable_fast_withdraw;
pub mod funding_wallet;
pub mod system_status;
pub mod trade_fee;
pub mod universal_transfer;
pub mod universal_transfer_history;
pub mod user_asset;
pub mod withdraw;
pub mod withdraw_history;

use rust_decimal::Decimal;

use account_snapshot::AccountSnapshot;
use account_status::AccountStatus;
use api_key_permission::APIKeyPermission;
use api_trading_status::APITradingStatus;
use asset_detail::AssetDetail;
use asset_dividend_record::AssetDividendRecord;
use coin_info::CoinInfo;
use deposit_address::DepositAddress;
use deposit_history::DepositHistory;
use disable_fast_withdraw::DisableFastWithdraw;
use dust_log::DustLog;
use dust_transfer::DustTransfer;
use dustable_assets::DustableAssets;
use enable_fast_withdraw::EnableFastWithdraw;
use funding_wallet::FundingWallet;
use system_status::SystemStatus;
use trade_fee::TradeFee;
use universal_transfer::UniversalTransfer;
use universal_transfer_history::UniversalTransferHistory;
use user_asset::UserAsset;
use withdraw::Withdraw;
use withdraw_history::WithdrawHistory;

pub fn system_status() -> SystemStatus {
    SystemStatus::new()
}

pub fn coin_info() -> CoinInfo {
    CoinInfo::new()
}

pub fn account_snapshot(r#type: &str) -> AccountSnapshot {
    AccountSnapshot::new(r#type)
}

pub fn disable_fast_withdraw() -> DisableFastWithdraw {
    DisableFastWithdraw::new()
}

pub fn enable_fast_withdraw() -> EnableFastWithdraw {
    EnableFastWithdraw::new()
}

pub fn withdraw(coin: &str, address: &str, amount: Decimal) -> Withdraw {
    Withdraw::new(coin, address, amount)
}

pub fn deposit_history() -> DepositHistory {
    DepositHistory::new()
}

pub fn withdraw_history() -> WithdrawHistory {
    WithdrawHistory::new()
}

pub fn deposit_address(coin: &str) -> DepositAddress {
    DepositAddress::new(coin)
}

pub fn account_status() -> AccountStatus {
    AccountStatus::new()
}

pub fn api_trading_status() -> APITradingStatus {
    APITradingStatus::new()
}

pub fn dust_log() -> DustLog {
    DustLog::new()
}

pub fn dustable_assets() -> DustableAssets {
    DustableAssets::new()
}

pub fn dust_transfer(asset: Vec<&str>) -> DustTransfer {
    DustTransfer::new(asset)
}

pub fn asset_dividend_record() -> AssetDividendRecord {
    AssetDividendRecord::new()
}

pub fn asset_detail() -> AssetDetail {
    AssetDetail::new()
}

pub fn trade_fee() -> TradeFee {
    TradeFee::new()
}

pub fn universal_transfer_history(r#type: &str) -> UniversalTransferHistory {
    UniversalTransferHistory::new(r#type)
}

pub fn universal_transfer(r#type: &str, asset: &str, amount: Decimal) -> UniversalTransfer {
    UniversalTransfer::new(r#type, asset, amount)
}

pub fn funding_wallet() -> FundingWallet {
    FundingWallet::new()
}

pub fn user_asset() -> UserAsset {
    UserAsset::new()
}

pub fn api_key_permission() -> APIKeyPermission {
    APIKeyPermission::new()
}
