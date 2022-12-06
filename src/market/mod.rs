//! Market Data

pub mod agg_trades;
pub mod avg_price;
pub mod book_ticker;
pub mod depth;
pub mod exchange_info;
pub mod historical_trades;
pub mod klines;
pub mod ping;
pub mod rolling_window_price_change_statistics;
pub mod ticker_price;
pub mod ticker_twenty_four_hr;
pub mod time;
pub mod trades;

use agg_trades::AggTrades;
use avg_price::AvgPrice;
use book_ticker::BookTicker;
use depth::Depth;
use exchange_info::ExchangeInfo;
use historical_trades::HistoricalTrades;
use klines::{KlineInterval, Klines};
use ping::Ping;
use rolling_window_price_change_statistics::RollingWindowPriceChangeStatistics;
use ticker_price::TickerPrice;
use ticker_twenty_four_hr::Ticker24hr;
use time::Time;
use trades::Trades;

pub fn ping() -> Ping {
    Ping::new()
}

pub fn time() -> Time {
    Time::new()
}

pub fn exchange_info() -> ExchangeInfo {
    ExchangeInfo::new()
}

pub fn depth(symbol: &str) -> Depth {
    Depth::new(symbol)
}

pub fn trades(symbol: &str) -> Trades {
    Trades::new(symbol)
}

pub fn historical_trades(symbol: &str) -> HistoricalTrades {
    HistoricalTrades::new(symbol)
}

pub fn agg_trades(symbol: &str) -> AggTrades {
    AggTrades::new(symbol)
}

pub fn klines(symbol: &str, interval: KlineInterval) -> Klines {
    Klines::new(symbol, interval)
}

pub fn avg_price(symbol: &str) -> AvgPrice {
    AvgPrice::new(symbol)
}

pub fn ticker_twenty_four_hr() -> Ticker24hr {
    Ticker24hr::new()
}

pub fn ticker_price() -> TickerPrice {
    TickerPrice::new()
}

pub fn book_ticker() -> BookTicker {
    BookTicker::new()
}

pub fn rolling_window_price_change_statistics() -> RollingWindowPriceChangeStatistics {
    RollingWindowPriceChangeStatistics::new()
}
