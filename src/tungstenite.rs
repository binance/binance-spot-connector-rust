use crate::websocket::Stream;
use std::io::{Read, Write};
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Error, Message, WebSocket};
use url::Url;

/// Binance websocket client using Tungstenite.
pub struct BinanceWebSocketClient;

impl BinanceWebSocketClient {
    pub fn connect_with_url(url: &str) -> Result<WebSocketState<MaybeTlsStream<TcpStream>>, Error> {
        let (socket, response) = connect(Url::parse(url).unwrap())?;

        log::info!("Connected to {}", url);
        log::debug!("Response HTTP code: {}", response.status());
        log::debug!("Response headers:");
        for (ref header, _value) in response.headers() {
            log::debug!("* {}", header);
        }

        Ok(WebSocketState::new(socket))
    }

    pub fn connect() -> Result<WebSocketState<MaybeTlsStream<TcpStream>>, Error> {
        BinanceWebSocketClient::connect_with_url("wss://stream.binance.com:9443/stream")
    }
}

pub struct WebSocketState<T> {
    socket: WebSocket<T>,
    id: u64,
}

impl<T: Read + Write> WebSocketState<T> {
    pub fn new(socket: WebSocket<T>) -> Self {
        Self { socket, id: 0 }
    }

    fn send<'a>(&mut self, method: &str, params: impl IntoIterator<Item = &'a str>) -> u64 {
        let mut params_str: String = params
            .into_iter()
            .map(|param| format!("\"{}\"", param))
            .collect::<Vec<String>>()
            .join(",");

        if !params_str.is_empty() {
            params_str = format!("\"params\": [{params}],", params = params_str)
        };

        let s = format!(
            "{{\"method\":\"{method}\",{params}\"id\":{id}}}",
            method = method,
            params = params_str,
            id = self.id
        );
        let message = Message::Text(s);
        log::debug!("Sent {}", message);

        self.socket.send(message).unwrap();

        self.id += 1;
        self.id
    }

    /// Sends `SUBSCRIBE` message for the given `streams`.
    ///
    /// `streams` are not validated. Invalid streams will be
    /// accepted by the server, but no data will be sent.
    /// Requests to subscribe an existing stream will be ignored
    /// by the server.
    ///
    /// Returns the message `id`. This should be used to match
    /// the request with a future response. Sent messages should
    /// not share the same message `id`.
    ///
    /// You should expect the server to respond with a similar
    /// message.
    /// ```json
    /// { "method": "SUBSCRIBE", "params": [ <streams> ], "id": <id> }
    /// ```
    pub fn subscribe<'a>(&mut self, streams: impl IntoIterator<Item = &'a Stream>) -> u64 {
        self.send("SUBSCRIBE", streams.into_iter().map(|s| s.as_str()))
    }

    /// Sends `UNSUBSCRIBE` message for the given `streams`.
    ///
    /// `streams` are not validated. Non-existing streams will be
    /// ignored by the server.
    ///
    /// Returns the message `id`. This should be used to match
    /// the request with a future response. Sent messages should
    /// not share the same message `id`.
    ///
    /// You should expect the server to respond with a similar
    /// message.
    /// ```json
    /// { "method": "UNSUBSCRIBE", "params": [ <streams> ], "id": <id> }
    /// ```
    pub fn unsubscribe<'a>(&mut self, streams: impl IntoIterator<Item = &'a Stream>) -> u64 {
        self.send("UNSUBSCRIBE", streams.into_iter().map(|s| s.as_str()))
    }

    /// Sends `LIST_SUBSCRIPTIONS` message.
    ///
    /// Returns the message `id`. This should be used to match
    /// the request with a future response. Sent messages should
    /// not share the same message `id`.
    ///
    /// You should expect the server to respond with a similar
    /// message.
    /// ```json
    /// { "method": "LIST_SUBSCRIPTIONS", "params": [ <streams> ], "id": <id> }
    /// ```
    pub fn subscriptions(&mut self) -> u64 {
        self.send("LIST_SUBSCRIPTIONS", vec![])
    }

    pub fn close(mut self) -> Result<(), Error> {
        self.socket.close(None)
    }
}

impl<T> From<WebSocketState<T>> for WebSocket<T> {
    fn from(conn: WebSocketState<T>) -> WebSocket<T> {
        conn.socket
    }
}

impl<T> AsMut<WebSocket<T>> for WebSocketState<T> {
    fn as_mut(&mut self) -> &mut WebSocket<T> {
        &mut self.socket
    }
}
