use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade,
};
use env_logger::Builder;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();

    // Hmac signature
    let the_api_key = "";
    let the_api_secret = "";
    let credentials = Credentials::from_hmac(the_api_key, the_api_secret);
    let client =
        BinanceHttpClient::with_url("https://testnet.binance.vision").credentials(credentials);
    let request = trade::account();
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);

    // Ed25519 signature
    let the_api_key = "";
    let private_key_path = "/Users/john/ssl/private_key_encrypted.pem";
    let private_key = fs::read_to_string(private_key_path).expect("Failed to read the private key");
    let credentials = Credentials::from_ed25519(the_api_key, private_key);
    let client =
        BinanceHttpClient::with_url("https://testnet.binance.vision").credentials(credentials);
    let request = trade::account();
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
