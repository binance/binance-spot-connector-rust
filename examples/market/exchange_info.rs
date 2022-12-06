use binance_spot_connector_rust::{
    hyper::{BinanceHttpClient, Error},
    market,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let testnet_http_client = BinanceHttpClient::with_url("https://testnet.binance.vision");
    let request = market::exchange_info().symbol("BNBUSDT");
    let data = testnet_http_client
        .send(request)
        .await?
        .into_body_str()
        .await?;
    log::info!("{}", data);
    Ok(())
}
