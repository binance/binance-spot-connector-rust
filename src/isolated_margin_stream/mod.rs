//! Market Data

pub mod close_listen_key;
pub mod new_listen_key;
pub mod renew_listen_key;

use close_listen_key::CloseListenKey;
use new_listen_key::NewListenKey;
use renew_listen_key::RenewListenKey;

pub fn new_listen_key(symbol: &str) -> NewListenKey {
    NewListenKey::new(symbol)
}

pub fn renew_listen_key(symbol: &str, listen_key: &str) -> RenewListenKey {
    RenewListenKey::new(symbol, listen_key)
}

pub fn close_listen_key(symbol: &str, listen_key: &str) -> CloseListenKey {
    CloseListenKey::new(symbol, listen_key)
}
