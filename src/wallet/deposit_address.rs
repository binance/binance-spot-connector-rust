use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/capital/deposit/address`
///
/// Fetch deposit address with network.
///
/// * If network is not send, return with default network of the coin.
/// * You can get network and isDefault in networkList in the response of Get /sapi/v1/capital/config/getall (HMAC SHA256).
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::deposit_address("BNB").network("ETH");
/// ```
pub struct DepositAddress {
    coin: String,
    network: Option<String>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl DepositAddress {
    pub fn new(coin: &str) -> Self {
        Self {
            coin: coin.to_owned(),
            network: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn network(mut self, network: &str) -> Self {
        self.network = Some(network.to_owned());
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

impl From<DepositAddress> for Request {
    fn from(request: DepositAddress) -> Request {
        let mut params = vec![("coin".to_owned(), request.coin.to_string())];

        if let Some(network) = request.network {
            params.push(("network".to_owned(), network));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/capital/deposit/address".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DepositAddress;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_deposit_address_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = DepositAddress::new("BNB")
            .network("ETH")
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/capital/deposit/address".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("coin".to_owned(), "BNB".to_string()),
                    ("network".to_owned(), "ETH".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
