use crate::market::klines::KlineInterval;
use crate::websocket::Stream;

/// Kline/Candlestick Streams for UTC
///
/// The Kline/Candlestick Stream push updates to the current klines/candlestick every second.
///
/// Update Speed: 2000ms
///
/// [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#klinecandlestick-streams-for-utc)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::{ market::klines::KlineInterval, market_stream::kline::KlineStream };
///
/// let stream = KlineStream::new("BTCUSDT", KlineInterval::Minutes1);
/// ```
pub struct KlineStream {
    symbol: String,
    interval: KlineInterval,
}

impl KlineStream {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            interval,
        }
    }
}

impl From<KlineStream> for Stream {
    /// Returns stream name as `<symbol>@kline_interval`
    fn from(stream: KlineStream) -> Stream {
        Stream::new(&format!("{}@kline_{}", stream.symbol, stream.interval))
    }
}
