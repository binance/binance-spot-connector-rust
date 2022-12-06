use crate::http::{request::Request, Method};

/// `GET /sapi/v1/system/status`
///
/// Fetch system status.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::system_status();
/// ```
pub struct SystemStatus {}

impl SystemStatus {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<SystemStatus> for Request {
    fn from(_request: SystemStatus) -> Request {
        let params = vec![];

        Request {
            path: "/sapi/v1/system/status".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SystemStatus;
    use crate::http::{request::Request, Method};

    #[test]
    fn wallet_system_status_convert_to_request_test() {
        let request: Request = SystemStatus::new().into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/system/status".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![],
                sign: false
            }
        );
    }
}
