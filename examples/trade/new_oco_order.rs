use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade::{
        self,
        order::{Side, TimeInForce},
    },
};
use env_logger::Builder;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let credentials = Credentials::from_hmac("api-key".to_owned(), "api-secret".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);
    let request = trade::new_oco_order("BNBUSDT", Side::Sell, dec!(0.1), dec!(400.15), dec!(390.3))
        .stop_limit_price(dec!(380.3))
        .stop_limit_time_in_force(TimeInForce::Gtc);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
