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
    let request = trade::new_oco_order(
        "BNBUSDT",
        Side::Sell,
        dec!(1.0),
        "LIMIT_MAKER",
        "STOP_LOSS_LIMIT",
    )
    .above_price(dec!(610.1))
    .below_price(dec!(600.3))
    .below_stop_price(dec!(598.2))
    .below_trailing_delta(dec!(60))
    .below_time_in_force(TimeInForce::Gtc);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
