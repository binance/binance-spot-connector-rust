use crate::http::{request::Request, Method};
use strum::Display;

#[derive(Copy, Clone, Display)]
#[strum(serialize_all = "UPPERCASE")]
pub enum SymbolStatus {
    Trading,
    Halt,
    Break,
}

/// `GET /api/v3/exchangeInfo`
///
/// Current exchange trading rules and symbol information
///
/// * If any symbol provided in either symbol or symbols do not exist, the endpoint will throw an error.
/// * Permissions can support single or multiple values (e.g. SPOT, ["MARGIN","LEVERAGED"]). This cannot be used in combination with symbol or symbols.
/// * If permissions parameter not provided, all symbols that have either SPOT, MARGIN, or LEVERAGED permission will be exposed.
///
/// Weight(IP): 20
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::market;
///
/// let request = market::exchange_info().symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct ExchangeInfo {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
    permissions: Option<Vec<String>>,
    show_permission_sets: Option<bool>,
    symbol_status: Option<SymbolStatus>,
}

impl ExchangeInfo {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
            permissions: None,
            show_permission_sets: None,
            symbol_status: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn symbols(mut self, symbols: Vec<&str>) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn permissions(mut self, permissions: Vec<&str>) -> Self {
        self.permissions = Some(permissions.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn show_permission_sets(mut self, show_permission_sets: bool) -> Self {
        self.show_permission_sets = Some(show_permission_sets);
        self
    }

    pub fn symbol_status(mut self, symbol_status: SymbolStatus) -> Self {
        self.symbol_status = Some(symbol_status);
        self
    }
}

impl From<ExchangeInfo> for Request {
    fn from(request: ExchangeInfo) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(symbols) = request.symbols {
            params.push((
                "symbols".to_owned(),
                format!("[\"{}\"]", symbols.join("\",\"")),
            ));
        }

        if let Some(permissions) = request.permissions {
            params.push((
                "permissions".to_owned(),
                format!("[\"{}\"]", permissions.join("\",\"")),
            ));
        }

        if let Some(show_permission_sets) = request.show_permission_sets {
            params.push((
                "showPermissionSets".to_owned(),
                show_permission_sets.to_string(),
            ));
        }

        if let Some(symbol_status) = request.symbol_status {
            params.push(("symbolStatus".to_owned(), symbol_status.to_string()));
        }

        Request {
            path: "/api/v3/exchangeInfo".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

impl Default for ExchangeInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::ExchangeInfo;
    use crate::http::{request::Request, Method};

    #[test]
    fn market_exchange_info_convert_to_request_test() {
        let request: Request = ExchangeInfo::new()
            .symbols(vec!["BTCUSDT", "BNBBTC"])
            .into();

        assert_eq!(
            request,
            Request {
                path: "/api/v3/exchangeInfo".to_owned(),
                credentials: None,
                method: Method::Get,
                params: vec![("symbols".to_owned(), "[\"BTCUSDT\",\"BNBBTC\"]".to_string())],
                sign: false
            }
        );
    }
}
