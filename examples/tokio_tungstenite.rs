use binance_spot_connector_rust::{
    market::klines::KlineInterval, market_stream::kline::KlineStream,
    tokio_tungstenite::BinanceWebSocketClient,
};
use env_logger::Builder;
use futures_util::StreamExt;
use std::time::Duration;

const BINANCE_WSS_BASE_URL: &str = "wss://stream.binance.com:9443/ws";

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    // Establish connection
    let (mut conn, _) = BinanceWebSocketClient::connect_async(BINANCE_WSS_BASE_URL)
        .await
        .expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &KlineStream::new("BTCUSDT", KlineInterval::Minutes1).into()
    ])
    .await;
    // Start a timer for 10 seconds
    let timer = tokio::time::Instant::now();
    let duration = Duration::new(10, 0);
    // Read messages
    while let Some(message) = conn.as_mut().next().await {
        if timer.elapsed() >= duration {
            log::info!("10 seconds elapsed, exiting loop.");
            break; // Exit the loop after 10 seconds
        }
        match message {
            Ok(message) => {
                let data = message.into_data();
                let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
                log::info!("{}", &string_data);
            }
            Err(_) => break,
        }
    }
    // Disconnect
    conn.close().await.expect("Failed to disconnect");
}
