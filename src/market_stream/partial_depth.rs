use crate::websocket::Stream;

/// Partial Book Depth Stream
///
/// Top bids and asks, Valid are 5, 10, or 20.
///
/// Update Speed: 1000ms or 100ms.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#partial-book-depth-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::partial_depth::PartialDepthStream;
///
/// let stream = PartialDepthStream::from_1000ms("BTCUSDT", 5);
/// let faster_update_speed_stream = PartialDepthStream::from_100ms("BTCUSDT", 5);
/// ```
pub struct PartialDepthStream {
    symbol: String,
    levels: u16,
    faster_update_speed: bool,
}

impl PartialDepthStream {
    pub fn from_1000ms(symbol: &str, levels: u16) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            levels,
            faster_update_speed: false,
        }
    }

    pub fn from_100ms(symbol: &str, levels: u16) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            levels,
            faster_update_speed: true,
        }
    }
}

impl From<PartialDepthStream> for Stream {
    /// Returns stream name as `<symbol>@depth<levels>` or `<symbol>@depth<levels>@100ms`
    fn from(stream: PartialDepthStream) -> Stream {
        if stream.faster_update_speed {
            Stream::new(&format!("{}@depth{}@100ms", stream.symbol, stream.levels))
        } else {
            Stream::new(&format!("{}@depth{}", stream.symbol, stream.levels))
        }
    }
}
