use crate::websocket::Stream;

/// Aggregate Trade Stream
///
/// The Aggregate Trade Streams push trade information that is aggregated for a single taker order.
///
/// Update Speed: Real-time.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#aggregate-trade-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::agg_trade::AggTradeStream;
///
/// let stream = AggTradeStream::new("BTCUSDT");
/// ```
pub struct AggTradeStream {
    symbol: String,
}

impl AggTradeStream {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
        }
    }
}

impl From<AggTradeStream> for Stream {
    /// Returns stream name as `<symbol>@aggTrade`
    fn from(stream: AggTradeStream) -> Stream {
        Stream::new(&format!("{}@aggTrade", stream.symbol))
    }
}
