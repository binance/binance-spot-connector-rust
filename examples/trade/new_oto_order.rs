use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade::{
        self,
        order::{Side, TimeInForce, WorkingMandatoryParams},
    },
};
use env_logger::Builder;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let credentials = Credentials::from_hmac("the_api_key".to_owned(), "the_api_secret".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);
    let request = trade::new_oto_order(
        "BNBUSDT",
        WorkingMandatoryParams::new("LIMIT", Side::Buy, dec!(596.0), dec!(1.0)),
        "LIMIT_MAKER",
        Side::Buy,
        dec!(1.0),
    )
    .working_time_in_force(TimeInForce::Gtc)
    .pending_price(dec!(598.1));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
