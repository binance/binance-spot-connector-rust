use crate::http::{request::Request, Credentials};
use crate::ureq::{Error, Response};
use crate::version::VERSION;
use http::Uri;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{Agent, AgentBuilder, Error as UreqError};

#[derive(Clone)]
pub struct BinanceHttpClient {
    client: Agent,
    base_url: String,
    timestamp_delta: i64,
    credentials: Option<Credentials>,
}

impl BinanceHttpClient {
    pub fn new(client: Agent, base_url: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    pub fn with_url(base_url: &str) -> Self {
        Self {
            client: AgentBuilder::new().build(),
            base_url: base_url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }

    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    pub fn timestamp_delta(mut self, timestamp_delta: i64) -> Self {
        self.timestamp_delta = timestamp_delta;
        self
    }
}

impl BinanceHttpClient {
    pub fn send<R: Into<Request>>(&self, request: R) -> Result<Response, Box<Error>> {
        let Request {
            method,
            path,
            params,
            credentials,
            sign,
        } = request.into();

        // Build URL
        let url: Uri = format!("{}{}", self.base_url, path).parse()?;

        let mut ureq_request = self.client.request(method.as_ref(), &url.to_string());

        // Set User-Agent in header
        let user_agent = &format!("binance-spot-connector-rust/{}", VERSION);
        ureq_request = ureq_request.set("User-Agent", user_agent);

        // Map query parameters
        let has_params = !params.is_empty();
        if has_params {
            for (k, v) in params.iter() {
                ureq_request = ureq_request.query(k, v);
            }
        }

        let client_credentials = self.credentials.as_ref();
        let request_credentials = credentials.as_ref();
        if let Some(Credentials { api_key, signature }) = request_credentials.or(client_credentials)
        {
            // Set API-Key in header
            ureq_request = ureq_request.set("X-MBX-APIKEY", api_key);

            if sign {
                // Use system clock, panic if system clock is behind `std::time::UNIX_EPOCH`
                let mut timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Clock may have gone backwards")
                    .as_millis();

                // Append timestamp delta to sync up with server time.
                timestamp -= self.timestamp_delta as u128;

                // Append timestamp to query parameters
                ureq_request = ureq_request.query("timestamp", &timestamp.to_string());

                // Stringfy available query parameters and append back to query parameters
                let signature = crate::utils::sign(
                    ureq_request
                        .request_url()
                        .unwrap()
                        .as_url()
                        .query()
                        .unwrap(),
                    signature,
                )
                .map_err(|_| Error::InvalidApiSecret)?;

                ureq_request = ureq_request.query("signature", &signature);
            }
        }

        log::debug!("{}", ureq_request.url());

        let response = match ureq_request.call() {
            Ok(response) => Ok(response),
            Err(UreqError::Status(_, response)) => Ok(response),
            Err(err) => Err(Error::Send(err)),
        }?;

        log::debug!("{}", response.status());

        Ok(Response::from(response))
    }
}

impl Default for BinanceHttpClient {
    fn default() -> Self {
        Self::new(AgentBuilder::new().build(), "https://api.binance.com")
    }
}

#[cfg(test)]
mod tests {
    use super::BinanceHttpClient;
    use crate::{
        http::{error::ClientError, request::Request, Credentials, Method},
        ureq::Error,
    };
    use std::collections::HashMap;
    use ureq::{
        AgentBuilder, Error as UreqError, Middleware, MiddlewareNext, Request as UreqRequest,
        Response,
    };

    #[test]
    fn client_respects_request_basic_configuration_test() {
        let agent = AgentBuilder::new()
            .middleware(
                MockMiddleware::new()
                    .method("GET")
                    .path("/path")
                    .base_url("https://base-url.com")
                    .param("testparam", Some("testparamvalue"))
                    .response(200, "Test Response"),
            )
            .build();

        let client = BinanceHttpClient::new(agent, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![("testparam".to_owned(), "testparamvalue".to_owned())],
            credentials: None,
            sign: false,
        };

        let data = client.send(request).unwrap().into_body_str().unwrap();

        assert_eq!(data, "Test Response".to_owned());
    }

    #[test]
    fn client_respects_request_credentials_no_signature_test() {
        let agent = AgentBuilder::new()
            .middleware(
                MockMiddleware::new()
                    .header("X-MBX-APIKEY", Some("api-key"))
                    .response(200, "Test Response"),
            )
            .build();

        let client = BinanceHttpClient::new(agent, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: Some(Credentials::from_hmac(
                "api-key".to_string(),
                "api-secret".to_string(),
            )),
            sign: false,
        };

        let data = client.send(request).unwrap().into_body_str().unwrap();

        assert_eq!(data, "Test Response".to_owned());
    }

    #[test]
    fn client_respects_request_credentials_with_signature_test() {
        let agent = AgentBuilder::new()
            .middleware(
                MockMiddleware::new()
                    .header("X-MBX-APIKEY", Some("api-key"))
                    .param("timestamp", None)
                    .param("signature", None)
                    .response(200, "Test Response"),
            )
            .build();

        let client = BinanceHttpClient::new(agent, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: Some(Credentials::from_hmac(
                "api-key".to_string(),
                "api-secret".to_string(),
            )),
            sign: true,
        };

        let data = client.send(request).unwrap().into_body_str().unwrap();

        assert_eq!(data, "Test Response".to_owned());
    }

    #[test]
    fn client_handles_not_found_error_test() {
        let agent = AgentBuilder::new()
            .middleware(MockMiddleware::new().response(404, ""))
            .build();

        let client = BinanceHttpClient::new(agent, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client.send(request).unwrap().into_body_str().unwrap_err();

        match *err {
            Error::Client(ClientError::Raw(err)) => assert_eq!(err.status_code, 404),
            _ => panic!("Invalid error"),
        }
    }

    #[test]
    fn client_handles_structured_bad_request_test() {
        let agent = AgentBuilder::new()
            .middleware(MockMiddleware::new()
            .response(400, "{ \"code\": -1102, \"msg\": \"Mandatory parameter 'symbol' was not sent, was empty/null, or malformed.\" }"))
            .build();

        let client = BinanceHttpClient::new(agent, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client.send(request).unwrap().into_body_str().unwrap_err();

        match *err {
            Error::Client(ClientError::Structured(err)) => assert_eq!(err.status_code, 400),
            _ => panic!("Invalid error"),
        }
    }

    #[test]
    fn client_handles_raw_bad_request_test() {
        let agent = AgentBuilder::new()
            .middleware(MockMiddleware::new().response(400, "Error"))
            .build();

        let client = BinanceHttpClient::new(agent, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client.send(request).unwrap().into_body_str().unwrap_err();

        match *err {
            Error::Client(ClientError::Raw(err)) => assert_eq!(err.status_code, 400),
            _ => panic!("Invalid error"),
        }
    }

    #[test]
    fn client_handles_server_error_test() {
        let agent = AgentBuilder::new()
            .middleware(MockMiddleware::new().response(500, "Error"))
            .build();

        let client = BinanceHttpClient::new(agent, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client.send(request).unwrap().into_body_str().unwrap_err();

        match *err {
            Error::Server(err) => assert_eq!(err.status_code, 500),
            _ => panic!("Invalid error"),
        }
    }

    struct MockMiddleware {
        base_url: Option<String>,
        path: Option<String>,
        method: Option<String>,
        params: Vec<(String, Option<String>)>,
        headers: HashMap<String, Option<String>>,
        response_code: Option<u16>,
        response_body: Option<String>,
    }

    impl MockMiddleware {
        pub fn new() -> Self {
            Self {
                base_url: None,
                path: None,
                method: None,
                params: vec![],
                headers: HashMap::new(),
                response_code: None,
                response_body: None,
            }
        }

        pub fn method(mut self, method: &str) -> Self {
            self.method = Some(method.to_string());
            self
        }

        pub fn param(mut self, name: &str, value: Option<&str>) -> Self {
            self.params
                .push((name.to_string(), value.map(|v| v.to_owned())));
            self
        }

        pub fn base_url(mut self, base_url: &str) -> Self {
            self.base_url = Some(base_url.to_string());
            self
        }

        pub fn path(mut self, path: &str) -> Self {
            self.path = Some(path.to_string());
            self
        }

        pub fn header(mut self, name: &str, value: Option<&str>) -> Self {
            self.headers
                .insert(name.to_string(), value.map(|v| v.to_owned()));
            self
        }

        pub fn response(mut self, code: u16, body: &str) -> Self {
            self.response_code = Some(code);
            self.response_body = Some(body.to_owned());
            self
        }
    }

    impl Middleware for MockMiddleware {
        fn handle(&self, req: UreqRequest, _: MiddlewareNext) -> Result<Response, UreqError> {
            let req_url = req.request_url()?;
            // Assert request method.
            let is_valid = self.method.as_ref().map(|m| m == req.method()).unwrap_or(true)
            // Assert request scheme and host.
            && self
                .base_url
                .as_ref()
                .map(|u| u == &format!("{}://{}", req_url.scheme(), req_url.host()))
                .unwrap_or(true)
            // Assert request path.
            && self
                .path
                .as_ref()
                .map(|p| p == req_url.path())
                .unwrap_or(true)
            // Assert request query parameters, if they exist.
            && self.params.iter().all(|(k, v)| {
                req_url.query_pairs()
                    .iter()
                    .any(|(pk, pv)| pk == k && v.as_ref().map(|v| v == pv).unwrap_or(true))
            })
            // Keys in `self.headers` exist in `headers` and their respective values
            // are equal.
            && self.headers.iter().all(|(k, v)| {
                req.header(k)
                    .map(|hv| v.as_ref().map(|v| hv == v).unwrap_or(true))
                    .unwrap_or(false)
            });
            if is_valid {
                Response::new(
                    self.response_code.unwrap_or(200),
                    "",
                    match &self.response_body {
                        Some(body) => body.as_str(),
                        None => "",
                    },
                )
            } else {
                Response::new(404, "Not Found", "")
            }
        }
    }
}
