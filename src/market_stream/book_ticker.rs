use crate::websocket::Stream;

/// Book Ticker Stream
///
/// Pushes any update to the best bid or ask's price or quantity in real-time for a specified symbol.
///
/// Update Speed: Real-time.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#individual-symbol-book-ticker-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::book_ticker::BookTickerStream;
///
/// let individual_symbol_stream = BookTickerStream::from_symbol("BTCUSDT");
/// let all_symbols_stream = BookTickerStream::all_symbols();
/// ```
pub struct BookTickerStream {
    symbol: Option<String>,
}

impl BookTickerStream {
    pub fn all_symbols() -> Self {
        Self { symbol: None }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        Self {
            symbol: Some(symbol.to_lowercase()),
        }
    }
}

impl From<BookTickerStream> for Stream {
    /// Returns stream name as `<symbol>@bookTicker` or `!bookTicker@arr`
    fn from(stream: BookTickerStream) -> Stream {
        if let Some(symbol) = stream.symbol {
            Stream::new(&format!("{}@bookTicker", symbol))
        } else {
            Stream::new("!bookTicker@arr")
        }
    }
}
