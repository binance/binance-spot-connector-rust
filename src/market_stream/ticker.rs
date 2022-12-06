use crate::websocket::Stream;

/// Ticker Stream
///
/// 24hr rolling window ticker statistics for a single symbol. These are NOT the statistics of the UTC day, but a 24hr rolling window for the previous 24hrs.
///
/// Update Speed: 1000ms.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#individual-symbol-ticker-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::ticker::TickerStream;
///
/// let individual_symbol_stream = TickerStream::from_symbol("BTCUSDT");
/// let all_symbols_stream = TickerStream::all_symbols();
/// ```
pub struct TickerStream {
    symbol: Option<String>,
}

impl TickerStream {
    pub fn all_symbols() -> Self {
        Self { symbol: None }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        Self {
            symbol: Some(symbol.to_lowercase()),
        }
    }
}

impl From<TickerStream> for Stream {
    /// Returns stream name as `<symbol>@ticker` or `!ticker@arr`
    fn from(stream: TickerStream) -> Stream {
        if let Some(symbol) = stream.symbol {
            Stream::new(&format!("{}@ticker", symbol))
        } else {
            Stream::new("!ticker@arr")
        }
    }
}
