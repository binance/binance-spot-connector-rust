use std::fmt;

/// Websocket stream.
///
/// The `Stream` trait is a simplified interface for Binance approved
/// websocket streams.
pub struct Stream {
    stream_name: String,
}

impl Stream {
    pub fn new(stream_name: &str) -> Self {
        Self {
            stream_name: stream_name.to_owned(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.stream_name
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.stream_name)
    }
}
