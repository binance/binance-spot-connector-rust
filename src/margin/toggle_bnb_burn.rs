use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/bnbBurn`
///
/// * "spotBNBBurn" and "interestBNBBurn" should be sent at least one.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::margin;
///
/// let request = margin::toggle_bnb_burn().spot_bnb_burn(true).interest_bnb_burn(false);
/// ```
pub struct ToggleBNBBurn {
    spot_bnb_burn: Option<bool>,
    interest_bnb_burn: Option<bool>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl ToggleBNBBurn {
    pub fn new() -> Self {
        Self {
            spot_bnb_burn: None,
            interest_bnb_burn: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn spot_bnb_burn(mut self, spot_bnb_burn: bool) -> Self {
        self.spot_bnb_burn = Some(spot_bnb_burn);
        self
    }

    pub fn interest_bnb_burn(mut self, interest_bnb_burn: bool) -> Self {
        self.interest_bnb_burn = Some(interest_bnb_burn);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<ToggleBNBBurn> for Request {
    fn from(request: ToggleBNBBurn) -> Request {
        let mut params = vec![];

        if let Some(spot_bnb_burn) = request.spot_bnb_burn {
            params.push(("spotBNBBurn".to_owned(), spot_bnb_burn.to_string()));
        }

        if let Some(interest_bnb_burn) = request.interest_bnb_burn {
            params.push(("interestBNBBurn".to_owned(), interest_bnb_burn.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/bnbBurn".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for ToggleBNBBurn {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ToggleBNBBurn;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn margin_toggle_bnb_burn_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = ToggleBNBBurn::new()
            .spot_bnb_burn(true)
            .interest_bnb_burn(false)
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/bnbBurn".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("spotBNBBurn".to_owned(), "true".to_string()),
                    ("interestBNBBurn".to_owned(), "false".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
