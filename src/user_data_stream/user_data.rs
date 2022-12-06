use crate::websocket::Stream;

/// User Data Stream.
///
/// A User Data Stream listenKey is valid for 60 minutes after creation.
///
/// Possible Updates:
///
/// * `outboundAccountPosition` is sent any time an account balance has
/// changed and contains the assets that were possibly changed by
/// the event that generated the balance change.
///
/// * `balanceUpdate` occurs during the following: Deposits or
/// withdrawals from the account; Transfer of funds between
/// accounts (e.g. Spot to Margin).
///
/// * `executionReport` occurs when an order is updated. If the order is
/// an OCO, an event will be displayed named `ListStatus` in addition
/// to the `executionReport` event.
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#user-data-streams)
pub struct UserDataStream {
    listen_key: String,
}

impl UserDataStream {
    pub fn new(listen_key: &str) -> Self {
        Self {
            listen_key: listen_key.to_owned(),
        }
    }
}

impl From<UserDataStream> for Stream {
    /// Returns stream name as `<listen_key>`
    fn from(stream: UserDataStream) -> Stream {
        Stream::new(&stream.listen_key)
    }
}
