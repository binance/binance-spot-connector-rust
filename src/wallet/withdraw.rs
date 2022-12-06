use crate::http::{request::Request, Credentials, Method};
use rust_decimal::Decimal;

/// `POST /sapi/v1/capital/withdraw/apply`
///
/// Submit a withdraw request.
///
/// * If `network` not send, return with default network of the coin.
/// * You can get `network` and `isDefault` in `networkList` of a coin in the response of `Get /sapi/v1/capital/config/getall (HMAC SHA256)`.
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
/// use rust_decimal_macros::dec;
///
/// let request = wallet::withdraw("BNB", "address", dec!(1.01));
/// ```
pub struct Withdraw {
    coin: String,
    address: String,
    amount: Decimal,
    withdraw_order_id: Option<String>,
    network: Option<String>,
    address_tag: Option<String>,
    transaction_fee_flag: Option<bool>,
    name: Option<String>,
    wallet_type: Option<u32>,
    recv_window: Option<u64>,
    credentials: Option<Credentials>,
}

impl Withdraw {
    pub fn new(coin: &str, address: &str, amount: Decimal) -> Self {
        Self {
            coin: coin.to_owned(),
            address: address.to_owned(),
            amount,
            withdraw_order_id: None,
            network: None,
            address_tag: None,
            transaction_fee_flag: None,
            name: None,
            wallet_type: None,
            recv_window: None,
            credentials: None,
        }
    }

    pub fn withdraw_order_id(mut self, withdraw_order_id: &str) -> Self {
        self.withdraw_order_id = Some(withdraw_order_id.to_owned());
        self
    }

    pub fn network(mut self, network: &str) -> Self {
        self.network = Some(network.to_owned());
        self
    }

    pub fn address_tag(mut self, address_tag: &str) -> Self {
        self.address_tag = Some(address_tag.to_owned());
        self
    }

    pub fn transaction_fee_flag(mut self, transaction_fee_flag: bool) -> Self {
        self.transaction_fee_flag = Some(transaction_fee_flag);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn wallet_type(mut self, wallet_type: u32) -> Self {
        self.wallet_type = Some(wallet_type);
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

impl From<Withdraw> for Request {
    fn from(request: Withdraw) -> Request {
        let mut params = vec![
            ("coin".to_owned(), request.coin.to_string()),
            ("address".to_owned(), request.address.to_string()),
            ("amount".to_owned(), request.amount.to_string()),
        ];

        if let Some(withdraw_order_id) = request.withdraw_order_id {
            params.push(("withdrawOrderId".to_owned(), withdraw_order_id));
        }

        if let Some(network) = request.network {
            params.push(("network".to_owned(), network));
        }

        if let Some(address_tag) = request.address_tag {
            params.push(("addressTag".to_owned(), address_tag));
        }

        if let Some(transaction_fee_flag) = request.transaction_fee_flag {
            params.push((
                "transactionFeeFlag".to_owned(),
                transaction_fee_flag.to_string(),
            ));
        }

        if let Some(name) = request.name {
            params.push(("name".to_owned(), name));
        }

        if let Some(wallet_type) = request.wallet_type {
            params.push(("walletType".to_owned(), wallet_type.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/sapi/v1/capital/withdraw/apply".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Withdraw;
    use crate::http::{request::Request, Credentials, Method};
    use rust_decimal_macros::dec;

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_withdraw_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = Withdraw::new("BNB", "address", dec!(1.01))
            .recv_window(5000)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/capital/withdraw/apply".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![
                    ("coin".to_owned(), "BNB".to_string()),
                    ("address".to_owned(), "address".to_string()),
                    ("amount".to_owned(), "1.01".to_string()),
                    ("recvWindow".to_owned(), "5000".to_string()),
                ],
                sign: true
            }
        );
    }
}
