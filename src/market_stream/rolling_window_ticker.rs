use crate::websocket::Stream;

/// Rolling Window Statistics Stream
///
/// Rolling window ticker statistics for a single symbol, computed over multiple windows.
///
/// Window Sizes: 1h,4h,1d
///
/// Update Speed: 1000ms.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#individual-symbol-rolling-window-statistics-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::rolling_window_ticker::RollingWindowTickerStream;
///
/// let individual_symbol_stream = RollingWindowTickerStream::from_symbol("BTCUSDT", "4h");
/// let all_symbols_stream = RollingWindowTickerStream::all_symbols("4h");
/// ```
pub struct RollingWindowTickerStream {
    window: String,
    symbol: Option<String>,
}

impl RollingWindowTickerStream {
    pub fn all_symbols(window: &str) -> Self {
        Self {
            symbol: None,
            window: window.to_string(),
        }
    }

    pub fn from_symbol(window: &str, symbol: &str) -> Self {
        Self {
            window: window.to_lowercase(),
            symbol: Some(symbol.to_lowercase()),
        }
    }
}

impl From<RollingWindowTickerStream> for Stream {
    /// Returns stream name as `<symbol>@ticker_<window>` or `!ticker_<window>@arr`
    fn from(stream: RollingWindowTickerStream) -> Stream {
        Stream::new(&if let Some(symbol) = stream.symbol {
            format!("{}@ticker_{}", symbol, stream.window)
        } else {
            format!("!ticker_{}@arr", stream.window)
        })
    }
}
