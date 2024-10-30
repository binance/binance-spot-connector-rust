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
    let request = trade::new_otoco_order(
        "BNBUSDT",
        WorkingMandatoryParams::new("LIMIT", Side::Sell, dec!(305), dec!(0.5)),
        Side::Sell,
        dec!(0.5),
        "LIMIT_MAKER",
    )
    .working_time_in_force(TimeInForce::Gtc)
    .pending_above_price(dec!(308))
    .pending_below_type("STOP_LOSS_LIMIT")
    .pending_below_stop_price(dec!(300.5))
    .pending_below_trailing_delta(dec!(30))
    .pending_below_time_in_force(TimeInForce::Gtc)
    .pending_below_price(dec!(301));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
