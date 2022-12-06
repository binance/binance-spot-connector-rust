//! Binance HTTP blocking client using ureq.
//!
//! # Usage
//!
//! ```no_run
//! use binance_spot_connector_rust::{ market::{self, klines::KlineInterval}, ureq::BinanceHttpClient };
//!
//! let client = BinanceHttpClient::default();
//!
//! let request = market::klines("BTCUSDT", KlineInterval::Minutes1);
//!
//! let data = client.send(request).expect("Failed to send request").into_body_str().expect("Failed to parse body");
//! ```
//!
//! # Testnet
//!
//! Can be configured to communicate with the testnet environment by specifying the base url on initialization.
//!
//! ```
//! use binance_spot_connector_rust::ureq::BinanceHttpClient;
//!
//! let testnet_client = BinanceHttpClient::with_url("https://testnet.binance.vision");
//! ```
//!
//! # Errors
//!
//! All errors emitted by the client can be converted into [Error].
//!
//! # Timeout
//!
//! ```
//! use ureq::{Agent, AgentBuilder};
//! use binance_spot_connector_rust::ureq::BinanceHttpClient;
//! use std::time::Duration;
//!
//! let agent: Agent = AgentBuilder::new()
//!   .timeout(Duration::from_secs(5))
//!   .build();
//!
//! let client = BinanceHttpClient::new(agent, "https://api.binance.com");
//! ```

mod client;
mod error;
mod response;

pub use client::*;
pub use error::*;
pub use response::*;
