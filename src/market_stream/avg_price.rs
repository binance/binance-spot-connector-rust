use crate::websocket::Stream;

/// Average Price
///
/// Average price streams push changes in the average price over a fixed time interval.
///
/// Update Speed: Real-time.
///
/// [API Documentation](https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#average-price)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market_stream::avg_price::AvgPriceStream;
///
/// let stream = AvgPriceStream::new("BNBUSDT");
/// ```
pub struct AvgPriceStream {
    symbol: String,
}

impl AvgPriceStream {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
        }
    }
}

impl From<AvgPriceStream> for Stream {
    /// Returns stream name as `<symbol>@avgPrice`
    fn from(stream: AvgPriceStream) -> Stream {
        Stream::new(&format!("{}@avgPrice", stream.symbol))
    }
}
