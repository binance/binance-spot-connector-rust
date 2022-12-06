use crate::websocket::Stream;

/// Diff. Depth Stream
///
/// Order book price and quantity depth updates used to locally manage an order book.
///
/// Update Speed: 1000ms or 100ms.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#partial-book-depth-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::diff_depth::DiffDepthStream;
///
/// let stream = DiffDepthStream::from_1000ms("BTCUSDT");
/// let faster_update_speed_stream = DiffDepthStream::from_100ms("BTCUSDT");
/// ```
pub struct DiffDepthStream {
    symbol: String,
    faster_update_speed: bool,
}

impl DiffDepthStream {
    pub fn from_1000ms(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            faster_update_speed: false,
        }
    }

    pub fn from_100ms(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            faster_update_speed: true,
        }
    }
}

impl From<DiffDepthStream> for Stream {
    /// Returns stream name as `<symbol>@depth` or `<symbol>@depth@100ms`
    fn from(stream: DiffDepthStream) -> Stream {
        if stream.faster_update_speed {
            Stream::new(&format!("{}@depth@100ms", stream.symbol))
        } else {
            Stream::new(&format!("{}@depth", stream.symbol))
        }
    }
}
