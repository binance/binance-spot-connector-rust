use crate::http::{request::Request, Credentials, Method};

/// `POST /sapi/v1/capital/deposit/credit-apply`
///
/// Apply deposit credit for expired address (One click arrival)
///
/// Weight(IP): 1
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::wallet;
///
/// let request = wallet::deposit_credit_apply().deposit_id(1);
/// ```
pub struct DepositCreditApply {
    deposit_id: Option<u64>,
    tx_id: Option<String>,
    sub_account_id: Option<u64>,
    sub_user_id: Option<u64>,
    credentials: Option<Credentials>,
}

impl DepositCreditApply {
    pub fn new() -> Self {
        Self {
            deposit_id: None,
            tx_id: None,
            sub_account_id: None,
            sub_user_id: None,
            credentials: None,
        }
    }

    pub fn deposit_id(mut self, deposit_id: u64) -> Self {
        self.deposit_id = Some(deposit_id);
        self
    }

    pub fn tx_id(mut self, tx_id: &str) -> Self {
        self.tx_id = Some(tx_id.to_owned());
        self
    }

    pub fn sub_account_id(mut self, sub_account_id: u64) -> Self {
        self.sub_account_id = Some(sub_account_id);
        self
    }

    pub fn sub_user_id(mut self, sub_user_id: u64) -> Self {
        self.sub_user_id = Some(sub_user_id);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<DepositCreditApply> for Request {
    fn from(request: DepositCreditApply) -> Request {
        let mut params = vec![];

        if let Some(deposit_id) = request.deposit_id {
            params.push(("depositId".to_owned(), deposit_id.to_string()));
        }

        if let Some(tx_id) = request.tx_id {
            params.push(("txId".to_owned(), tx_id));
        }

        if let Some(sub_account_id) = request.sub_account_id {
            params.push(("subAccountId".to_owned(), sub_account_id.to_string()));
        }

        if let Some(sub_user_id) = request.sub_user_id {
            params.push(("subUserId".to_owned(), sub_user_id.to_string()));
        }

        Request {
            path: "/sapi/v1/capital/deposit/credit-apply".to_owned(),
            method: Method::Post,
            params,
            credentials: request.credentials,
            sign: true,
        }
    }
}

impl Default for DepositCreditApply {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::DepositCreditApply;
    use crate::http::{request::Request, Credentials, Method};

    static API_KEY: &str = "api-key";
    static API_SECRET: &str = "api-secret";

    #[test]
    fn wallet_deposit_credit_apply_convert_to_request_test() {
        let credentials = Credentials::from_hmac(API_KEY.to_owned(), API_SECRET.to_owned());

        let request: Request = DepositCreditApply::new()
            .deposit_id(1)
            .credentials(&credentials)
            .into();

        assert_eq!(
            request,
            Request {
                path: "/sapi/v1/capital/deposit/credit-apply".to_owned(),
                credentials: Some(credentials),
                method: Method::Post,
                params: vec![("depositId".to_owned(), "1".to_owned())],
                sign: true
            }
        );
    }
}
