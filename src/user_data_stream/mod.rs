//! Binance SPOT User Data Websocket  Streams
//!
//! A collection of SPOT User Data Websocket streams.
mod user_data;

pub use user_data::UserDataStream;

pub fn user_data(listen_key: &str) -> UserDataStream {
    UserDataStream::new(listen_key)
}
