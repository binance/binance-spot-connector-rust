use crate::websocket::Stream;

/// Aggregate Trade Stream
///
/// The Trade Streams push raw trade information; each trade has a unique buyer and seller.
///
/// Update Speed: Real-time.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#trade-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::trade::TradeStream;
///
/// let stream = TradeStream::new("BTCUSDT");
/// ```
pub struct TradeStream {
    symbol: String,
}

impl TradeStream {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
        }
    }
}

impl From<TradeStream> for Stream {
    /// Returns stream name as `<symbol>@trade`
    fn from(stream: TradeStream) -> Stream {
        Stream::new(&format!("{}@trade", stream.symbol))
    }
}
