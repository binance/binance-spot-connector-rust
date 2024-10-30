use crate::http::{request::Request, Credentials, Method};

/// `GET /sapi/v1/capital/deposit/address/list`
///
/// Fetch deposit address list with network.
///
/// * If network is not send, return with default network of the coin.
/// * You can get network and isDefault in networkList in the response of `Get /sapi/v1/capital/config/getall`.
///
/// Weight(IP): 10
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::deposit_address_list("BNB").network("ETH");
/// ```
pub struct DepositAddressList {
    coin: String,
    network: Option<String>,
    credentials: Option<Credentials>,
}

impl DepositAddressList {
    pub fn new(coin: &str) -> Self {
        Self {
            coin: coin.to_owned(),
            network: None,
            credentials: None,
        }
    }

    pub fn network(mut self, network: &str) -> Self {
        self.network = Some(network.to_owned());
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<DepositAddressList> for Request {
    fn from(request: DepositAddressList) -> Request {
        let mut params = vec![("coin".to_owned(), request.coin.to_string())];

        if let Some(network) = request.network {
            params.push(("network".to_owned(), network));
        }

        Request {
            path: "/sapi/v1/capital/deposit/address/list".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DepositAddressList;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_deposit_address_list_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = DepositAddressList::new("BNB")
            .network("ETH")
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/capital/deposit/address/list".to_owned(),
                credentials: Some(credentials),
                method: Method::Get,
                params: vec![
                    ("coin".to_owned(), "BNB".to_string()),
                    ("network".to_owned(), "ETH".to_string()),
                ],
                sign: true
            }
        );
    }
}
