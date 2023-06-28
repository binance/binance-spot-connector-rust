//! A lightweight library to integrate with the Binance Spot API.
//!
//! * Spot REST API endpoints (`/api/*` and `/sapi/*`)
//! * Spot Market and User Data web-socket streams.
//! * Spot Testnet and Production environments.
//! * Tests and examples
//! * Blocking and non-blocking http clients
//! * Generic http request framework for custom requests.
//!
//! ## Features
//!
//! By default, **binance-spot** only enables blocking clients to minimize the size of compiled code.
//! To enable other clients, use the named features below when adding the crate as a dependancy.
//! To include all features, you may use the `all` feature.
//!
//! The following optional features are available:
//!
//! * `enable-ureq`: For a blocking http client powered by [`ureq`](https://docs.rs/ureq/2.4.0/ureq/).
//! * `enable-hyper`: For a non-blocking http client powered by [`hyper`](https://docs.rs/hyper/0.14.16/hyper/).
//! * `enable-tungstenite`: For a blocking web-socket client powered by [`tungstenite`](https://docs.rs/tungstenite/0.16.0/tungstenite/).
//! * `enable-tokio-tungstenite`: For a non-blocking web-socket client powered by [`tokio-tungstenite`](https://docs.rs/tokio-tungstenite/0.17.1/tokio_tungstenite/).
//!
//! # Testnet
//!
//! Http clients and web-socket clients can be configured to communicate with the testnet environment by specifying the base url on initialization.
//!
//! ```
//! use binance_spot_connector_rust::ureq::BinanceHttpClient;
//!
//! let testnet_http_client = BinanceHttpClient::with_url("https://testnet.binance.vision");
//! ```
//!
//! ```no_run
//! use binance_spot_connector_rust::tungstenite::BinanceWebSocketClient;
//!
//! let testnet_websocket_client = BinanceWebSocketClient::connect_with_url("wss://testnet.binance.vision/ws");
//! ```
//!
//! # Logging
//!
//! Internally, the crate invokes log requests using the official [`log`](https://docs.rs/log/0.4.14/log/) framework to give visibility
//! to the requests' url and their respective response status code.
//!
//! Here is a snippet requesting the current average price for BNBUSDT using [`env_logger`](https://docs.rs/env_logger/0.9.0/env_logger/).
//!
//! ```no_run
//! use binance_spot_connector_rust::{market, ureq::BinanceHttpClient};
//! use env_logger::Builder;
//!
//! Builder::from_default_env()
//!     .filter_level(log::LevelFilter::Info)
//!     .init();
//!
//! let client = BinanceHttpClient::default();
//!
//! let response = client.send(market::historical_trades("BNBUSDT"));
//!
//! let body = response.expect("Request failed").into_body_str().expect("Failed to parse body");
//!
//! log::info!("{}", body);
//! ```
//!
//! Generated logs.
//! ```ignore
//! [2022-01-01T00:00:01Z INFO  binance_spot_connector_rust::ureq::client] https://api.binance.com/api/v3/avgPrice?symbol=BNBUSDT
//! [2022-01-01T00:00:01Z INFO  binance_spot_connector_rust::ureq::client] 200
//! [2022-01-01T00:00:01Z INFO  market_current_avg_price] {"mins":5,"price":"306.08482159"}
//! ```
//!
//! To ignore log requests by the library use `env_logger::Builder::filter`. If no loggger is configured, the library's internal log requests will be ignored.
//!
//! ```ignore
//! builder.filter(Some("binance_spot_connector_rust"), log::LevelFilter::Off)
//! ```
//!
//! # Errors
//!
//! All errors emitted by the library can be converted to the generic error type of the respective client used to avoid complex error handling when using the pre-defined clients.

mod utils;
mod version;
mod websocket;

#[cfg(feature = "enable-tokio-tungstenite")]
pub mod tokio_tungstenite;

#[cfg(feature = "enable-tungstenite")]
pub mod tungstenite;

#[cfg(feature = "enable-hyper")]
pub mod hyper;

#[cfg(feature = "enable-ureq")]
pub mod ureq;

pub mod http;

pub mod isolated_margin_stream;
pub mod margin_stream;
pub mod market_stream;
pub mod stream;
pub mod user_data_stream;

pub mod margin;
pub mod market;
pub mod trade;
pub mod wallet;
