use crate::http::{request::Request, Method};

/// `GET /api/v3/time`
///
/// Test connectivity to the Rest API and get the current server time.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::time();
/// ```
pub struct Time {}

impl Time {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<Time> for Request {
    fn from(_request: Time) -> Request {
        let params = vec![];

        Request {
            path: "/api/v3/time".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Time;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_time_convert_to_request_test() {
        let request: Request = Time::new().into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/time".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![],
                sign: false
            }
        );
    }
}
