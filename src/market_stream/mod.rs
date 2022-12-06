//! Binance SPOT Market Websocket Streams
//!
//! A collection of SPOT Market Websocket streams.
pub mod agg_trade;
pub mod book_ticker;
pub mod diff_depth;
pub mod kline;
pub mod mini_ticker;
pub mod partial_depth;
pub mod rolling_window_ticker;
pub mod ticker;
pub mod trade;

use crate::market::klines::KlineInterval;

use agg_trade::AggTradeStream;
use book_ticker::BookTickerStream;
use diff_depth::DiffDepthStream;
use kline::KlineStream;
use mini_ticker::MiniTickerStream;
use partial_depth::PartialDepthStream;
use rolling_window_ticker::RollingWindowTickerStream;
use ticker::TickerStream;
use trade::TradeStream;

pub fn agg_trades(symbol: &str) -> AggTradeStream {
    AggTradeStream::new(symbol)
}

pub fn individual_symbol_book_ticker(symbol: &str) -> BookTickerStream {
    BookTickerStream::from_symbol(symbol)
}

pub fn all_market_book_ticker() -> BookTickerStream {
    BookTickerStream::all_symbols()
}

pub fn diff_depth_1000ms(symbol: &str) -> DiffDepthStream {
    DiffDepthStream::from_1000ms(symbol)
}

pub fn diff_depth_100ms(symbol: &str) -> DiffDepthStream {
    DiffDepthStream::from_100ms(symbol)
}

pub fn klines(symbol: &str, interval: KlineInterval) -> KlineStream {
    KlineStream::new(symbol, interval)
}

pub fn individual_symbol_mini_ticker(symbol: &str) -> MiniTickerStream {
    MiniTickerStream::from_symbol(symbol)
}

pub fn all_market_mini_ticker() -> MiniTickerStream {
    MiniTickerStream::all_symbols()
}

pub fn partial_depth_1000ms(symbol: &str, levels: u16) -> PartialDepthStream {
    PartialDepthStream::from_1000ms(symbol, levels)
}

pub fn partial_depth_100ms(symbol: &str, levels: u16) -> PartialDepthStream {
    PartialDepthStream::from_100ms(symbol, levels)
}

pub fn individual_symbol_rolling_window_ticker(
    symbol: &str,
    window: &str,
) -> RollingWindowTickerStream {
    RollingWindowTickerStream::from_symbol(symbol, window)
}

pub fn all_market_rolling_window_ticker(window: &str) -> RollingWindowTickerStream {
    RollingWindowTickerStream::all_symbols(window)
}

pub fn individual_symbol_ticker(symbol: &str) -> TickerStream {
    TickerStream::from_symbol(symbol)
}

pub fn all_market_ticker() -> TickerStream {
    TickerStream::all_symbols()
}

pub fn trade_stream(symbol: &str) -> TradeStream {
    TradeStream::new(symbol)
}
