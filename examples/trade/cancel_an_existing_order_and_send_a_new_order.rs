use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade::{
        self,
        order::{CancelReplaceMode, Side, TimeInForce},
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
    let request = trade::cancel_an_existing_order_and_send_a_new_order(
        "BNBUSDT",
        Side::Sell,
        "LIMIT",
        CancelReplaceMode::StopOnFailure,
    )
    .time_in_force(TimeInForce::Gtc)
    .quantity(dec!(10.1))
    .price(dec!(295.92))
    .cancel_order_id(12)
    .stop_price(dec!(20.01));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
