use binance_spot_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market::{self, rolling_window_price_change_statistics::TickerType},
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();
    let request = market::ticker_trading_day()
        .symbol("BNBUSDT")
        .ticker_type(TickerType::Mini);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
