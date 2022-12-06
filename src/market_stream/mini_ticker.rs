use crate::websocket::Stream;

/// Mini Ticker Stream
///
/// 24hr rolling window mini-ticker statistics. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.
///
/// Update Speed: 1000ms.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#individual-symbol-mini-ticker-stream)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::mini_ticker::MiniTickerStream;
///
/// let individual_symbol_stream = MiniTickerStream::from_symbol("BTCUSDT");
/// let all_symbols_stream = MiniTickerStream::all_symbols();
/// ```
pub struct MiniTickerStream {
    symbol: Option<String>,
}

impl MiniTickerStream {
    pub fn all_symbols() -> Self {
        Self { symbol: None }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        Self {
            symbol: Some(symbol.to_lowercase()),
        }
    }
}

impl From<MiniTickerStream> for Stream {
    /// Returns stream name as `<symbol>@miniTicker` or `!miniTicker@arr`
    fn from(stream: MiniTickerStream) -> Stream {
        if let Some(symbol) = stream.symbol {
            Stream::new(&format!("{}@miniTicker", symbol))
        } else {
            Stream::new("!miniTicker@arr")
        }
    }
}
