use crate::websocket::Stream;
use futures_util::SinkExt;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::Response, protocol::Message, Error},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

/// Binance websocket client using Tungstenite.
pub struct BinanceWebSocketClient;

impl BinanceWebSocketClient {
    pub async fn connect_async(
        url: &str,
    ) -> Result<(WebSocketState<MaybeTlsStream<TcpStream>>, Response), Error> {
        let (socket, response) = connect_async(Url::parse(&url).unwrap()).await?;

        log::info!("Connected to {}", url);
        log::debug!("Response HTTP code: {}", response.status());
        log::debug!("Response headers:");
        for (ref header, _value) in response.headers() {
            log::debug!("* {}", header);
        }

        Ok((WebSocketState::new(socket), response))
    }

    pub async fn connect_async_default(
    ) -> Result<(WebSocketState<MaybeTlsStream<TcpStream>>, Response), Error> {
        BinanceWebSocketClient::connect_async("wss://stream.binance.com:9443/stream").await
    }
}

pub struct WebSocketState<T> {
    socket: WebSocketStream<T>,
    id: u64,
}

impl<T: AsyncRead + AsyncWrite + Unpin> WebSocketState<T> {
    pub fn new(socket: WebSocketStream<T>) -> Self {
        Self { socket, id: 0 }
    }

    async fn send(&mut self, method: &str, params: impl IntoIterator<Item = &str>) -> u64 {
        let mut params_str: String = params
            .into_iter()
            .map(|param| format!("\"{}\"", param))
            .collect::<Vec<String>>()
            .join(",");

        if !params_str.is_empty() {
            params_str = format!("\"params\": [{params}],", params = params_str)
        };

        let id = self.id.clone();
        self.id += 1;

        let s = format!(
            "{{\"method\":\"{method}\",{params}\"id\":{id}}}",
            method = method,
            params = params_str,
            id = id
        );
        let message = Message::Text(s);
        log::debug!("Sent {}", message);

        self.socket.send(message).await.unwrap();

        id
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
    pub async fn subscribe(&mut self, streams: impl IntoIterator<Item = &Stream>) -> u64 {
        self.send("SUBSCRIBE", streams.into_iter().map(|s| s.as_str()))
            .await
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
    pub async fn unsubscribe(&mut self, streams: impl IntoIterator<Item = &Stream>) -> u64 {
        self.send("UNSUBSCRIBE", streams.into_iter().map(|s| s.as_str()))
            .await
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
    pub async fn subscriptions(&mut self) -> u64 {
        self.send("LIST_SUBSCRIPTIONS", vec![]).await
    }

    pub async fn close(mut self) -> Result<(), Error> {
        self.socket.close(None).await
    }
}

impl<T> From<WebSocketState<T>> for WebSocketStream<T> {
    fn from(conn: WebSocketState<T>) -> WebSocketStream<T> {
        conn.socket
    }
}

impl<T> AsMut<WebSocketStream<T>> for WebSocketState<T> {
    fn as_mut(&mut self) -> &mut WebSocketStream<T> {
        &mut self.socket
    }
}
