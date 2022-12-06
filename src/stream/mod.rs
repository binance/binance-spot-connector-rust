//! Market Data

pub mod close_listen_key;
pub mod new_listen_key;
pub mod renew_listen_key;

use close_listen_key::CloseListenKey;
use new_listen_key::NewListenKey;
use renew_listen_key::RenewListenKey;

pub fn new_listen_key() -> NewListenKey {
    NewListenKey::new()
}

pub fn renew_listen_key(listen_key: &str) -> RenewListenKey {
    RenewListenKey::new(listen_key)
}

pub fn close_listen_key(listen_key: &str) -> CloseListenKey {
    CloseListenKey::new(listen_key)
}
