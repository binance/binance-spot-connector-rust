use crate::http::{request::Request, Method};

/// `GET /api/v3/ping`
///
/// Test connectivity to the Rest API.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::ping();
/// ```
pub struct Ping {}

impl Ping {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<Ping> for Request {
    fn from(_request: Ping) -> Request {
        let params = vec![];

        Request {
            path: "/api/v3/ping".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for Ping {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Ping;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_ping_convert_to_request_test() {
        let request: Request = Ping::new().into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/ping".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![],
                sign: false
            }
        );
    }
}
