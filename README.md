# Binance Public API Connector Rust

This is a lightweight library that works as a connector to [Binance public API](https://github.com/binance/binance-spot-api-docs)

- Supported APIs:
  - `/api/*`
  - `/sapi/*`
  - Spot Websocket Market Stream
  - Spot User Data Stream
- Test cases and examples
- Customizable base URL and request timeout
- Blocking and Non-blocking clients
- Response Metadata

## RESTful APIs

The Binance Rust Connector exposes two abstraction layers to integrete with Binance RESTful APIs; a high level
abstraction consisting of maintained functions mapped one-to-one with Binance API endpoints, and a low level
generic abstraction for more control over the request.

**High Level Usage Example**

```rust
use env_logger::Builder;
use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    market::{self, klines::KlineInterval},
    trade
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let credentials = Credentials::from_hmac("api-key".to_owned(), "api-secret".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);

    // Get candlesticks for BTCUSDT with a 1 minute interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Minutes1)).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("{}", data);

    // Get the last 10 candlesticks for BTCUSDT with a 1 hour interval
    let data = client.send(market::klines("BTCUSDT", KlineInterval::Hours1).limit(10)).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("{}", data);

    // Get account information
    let data = client.send(trade::account()).await
        .expect("Request failed")
        .into_body_str().await
        .expect("Failed to read response body");
    log::info!("{}", data);

    Ok(())
}
```

Examples for other endpoints are available in the `examples` folder.

**Low Level Usage Example**

```rust
use binance_spot_connector_rust::{
    http::{request::RequestBuilder, Method},
    hyper::{BinanceHttpClient, Error},
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();

    let builder = RequestBuilder::new(Method::Get, "/api/v3/klines")
        .params(vec![("symbol", "BTCUSDT"), ("interval", "1m")]);

    // Get candlesticks for BTCUSDT with a 1 minute interval
    let data = client
        .send(builder)
        .await
        .expect("Request failed")
        .into_body_str()
        .await
        .expect("Failed to read response body");
    log::info!("{}", data);

    Ok(())
}
```

## Websocket

```rust
use binance_spot_connector_rust::{
    market::klines::KlineInterval, market_stream::kline::KlineStream,
    tokio_tungstenite::BinanceWebSocketClient,
};
use env_logger::Builder;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    // Establish connection
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &KlineStream::new("BTCUSDT", KlineInterval::Minutes1).into()
    ])
    .await;
    // Read messages
    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                let binary_data = message.into_data();
                let data = std::str::from_utf8(&binary_data).expect("Failed to parse message");
                log::info!("{:?}", data);
            }
            Err(_) => break,
        }
    }
    // Disconnect
    conn.close().await.expect("Failed to disconnect");
}

```

Examples for other websocket streams are available in the `examples` folder.

### Heartbeat

Once connected, the websocket server sends a ping frame every 3 minutes and requires a response pong frame back within
a 10 minutes period. This package handles the pong responses automatically.

### Testnet

`/api/*` endpoints can be tested in [Spot Testnet](https://testnet.binance.vision/). `/sapi/*` endpoints are not supported.

```rust
let client = BinanceHttpClient::with_url("https://testnet.binance.vision");
```

### Base URL

It's recommended to pass in the `baseUrl` parameter, even in production as Binance provides alternative URLs
in case of performance issues:

- `https://api1.binance.com`
- `https://api2.binance.com`
- `https://api3.binance.com`

### Timeout

The default timeout is 100,000 milliseconds (100 seconds).

### Logging

This library implements the standard rust logging framework which works with a variety of built-in and third-party logging providers.

For more information on how to configure logging in Rust, visit [Rust Log](https://docs.rs/log/latest/log/)

**Usage Example**

```rust
use binance_spot_connector_rust::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    wallet,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let credentials = Credentials::from_hmac("api-key".to_owned(), "api-secret".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);

    // Get candlesticks for BTCUSDT with a 1 minute interval
    let data = client
        .send(wallet::system_status())
        .await
        .expect("Request failed")
        .into_body_str()
        .await
        .expect("Failed to read response body");

    Ok(())
}
```

**Sample Output**

```
[2022-02-22T00:00:00Z DEBUG binance_spot_connector_rust::hyper::client] https://api.binance.com/sapi/v1/system/status
[2022-02-22T00:00:00Z DEBUG binance_spot_connector_rust::hyper::client] 200 OK
[2022-02-22T00:00:00Z INFO  binance_spot_connector_rust::hyper::client] {"status":0,"msg":"normal"}
```

## Test Cases

```bash
cargo test
```

## Examples

All snippets for spot endpoints and streams can be found in the `examples` folder. Example names are a concatanation of the example directory name and the example file name separated by an underscore.

```bash
cd examples && cargo run --example market_exchange_info
```

## Limitations

Futures and Vanilla Options APIs are not supported:

- /fapi/\*
- /dapi/\*
- /vapi/\*
- Associated Websocket Market and User Data Streams

## Contributing

Contributions are welcome.

If you've found a bug within this project, please open an issue to discuss what you would like to change.

If it's an issue with the API, please open a topic at [Binance Developer Community](https://dev.binance.vision)
