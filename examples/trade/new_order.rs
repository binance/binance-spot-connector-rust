use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade::{self, order::Side},
};
use env_logger::Builder;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    let credentials = Credentials::from_hmac("the_api_key".to_owned(), "the_api_secret".to_owned());
    let client =
        BinanceHttpClient::with_url("https://testnet.binance.vision").credentials(credentials);
    let request = trade::new_order("BNBUSDT", Side::Sell, "MARKET").quantity(dec!(0.1));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
