use binance_spot_connector_rust::{
    market::klines::KlineInterval, market_stream::kline::KlineStream,
    tungstenite::BinanceWebSocketClient,
};
use env_logger::Builder;

const BINANCE_WSS_BASE_URL: &str = "wss://stream.binance.com:9443/ws";

fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    // Establish connection
    let mut conn =
        BinanceWebSocketClient::connect_with_url(BINANCE_WSS_BASE_URL).expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &KlineStream::new("BTCUSDT", KlineInterval::Minutes1).into(),
        &KlineStream::new("BNBBUSD", KlineInterval::Minutes3).into(),
    ]);
    // Read messages
    while let Ok(message) = conn.as_mut().read() {
        let data = message.into_data();
        let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
        log::info!("{}", &string_data);
    }
    // Disconnect
    conn.close().expect("Failed to disconnect");
}
