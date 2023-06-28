use crate::http::{request::Request, Credentials, Method};
use crate::hyper::{Error, Response};
use crate::version::VERSION;
use hyper::{client::connect::Connect, client::HttpConnector, Body, Client, Uri};
use hyper_tls::HttpsConnector;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct BinanceHttpClient<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    client: Client<T, Body>,
    base_url: String,
    timestamp_delta: i64,
    credentials: Option<Credentials>,
}

impl<T> BinanceHttpClient<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub fn new(client: Client<T, Body>, base_url: &str) -> Self {
        Self {
            client,
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

impl BinanceHttpClient<HttpsConnector<HttpConnector>> {
    pub fn with_url(base_url: &str) -> BinanceHttpClient<HttpsConnector<HttpConnector>> {
        BinanceHttpClient {
            client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
            base_url: base_url.to_owned(),
            timestamp_delta: 0,
            credentials: None,
        }
    }
}

impl<T> BinanceHttpClient<T>
where
    T: Connect + Clone + Send + Sync + 'static,
{
    pub async fn send<R: Into<Request>>(&self, request: R) -> Result<Response, Error> {
        let Request {
            method,
            path,
            params,
            credentials,
            sign,
        } = request.into();
        let mut url_parts = vec![self.base_url.to_owned(), path];
        let has_params = !params.is_empty();
        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        if has_params {
            for (k, v) in params.iter() {
                serializer.append_pair(k, v);
            }
        }
        let mut query_string = serializer.finish();
        let mut hyper_request = hyper::Request::builder().method(method);
        let user_agent = &format!("binance-spot-connector-rust/{}", VERSION);
        hyper_request = hyper_request.header("User-Agent", user_agent);
        let client_credentials = self.credentials.as_ref();
        let request_credentials = credentials.as_ref();
        if let Some(Credentials { api_key, signature }) = request_credentials.or(client_credentials)
        {
            hyper_request = hyper_request.header("X-MBX-APIKEY", api_key);
            if sign {
                let mut timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Clock may have gone backwards")
                    .as_millis();
                timestamp -= self.timestamp_delta as u128;

                if has_params {
                    query_string.push_str(format!("&timestamp={}", timestamp).as_str());
                } else {
                    query_string.push_str(format!("timestamp={}", timestamp).as_str());
                }

                let signature = crate::utils::sign(&query_string, signature)
                    .map_err(|_| Error::InvalidApiSecret)?;
                let encoded_signature: String =
                    url::form_urlencoded::byte_serialize(signature.as_bytes()).collect();
                query_string.push_str(format!("&signature={}", encoded_signature).as_str());
            }
        }

        if !query_string.is_empty() {
            url_parts.push(String::from("?"));
            url_parts.push(query_string);
        }
        let uri: Uri = url_parts.join("").parse()?;
        log::debug!("{}", uri);
        let hyper_request = hyper_request.uri(uri);
        let request = hyper_request
            .body(Body::empty())
            .map_err(|err| Error::Parse(err))?;
        let response = self
            .client
            .request(request)
            .await
            .map_err(|err| Error::Send(err))?;
        log::debug!("{}", response.status());

        Ok(Response::from(response))
    }
}

impl Default for BinanceHttpClient<HttpsConnector<HttpConnector>> {
    fn default() -> Self {
        Self::new(
            Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
            "https://api.binance.com",
        )
    }
}

impl From<Method> for hyper::Method {
    fn from(method: Method) -> hyper::Method {
        match method {
            Method::Post => hyper::Method::POST,
            Method::Delete => hyper::Method::DELETE,
            Method::Get => hyper::Method::GET,
            Method::Put => hyper::Method::PUT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BinanceHttpClient;
    use crate::http::{error::ClientError, request::Request, Credentials, Method};
    use crate::hyper::Error;
    use hyper::client::connect::Connected;
    use hyper::{Client, Uri};
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::str;
    use std::task::{Context, Poll, Waker};
    use tokio::io::Error as IoError;
    use tokio::io::ReadBuf;

    #[tokio::test]
    async fn client_respects_request_basic_configuration_test() {
        let client = Client::builder().build(
            MockConnector::new()
                .method("GET")
                .path("/path")
                .base_url("https://base-url.com")
                .param("testparam", Some("testparamvalue"))
                .response(200, "Test Response"),
        );
        let client = BinanceHttpClient::new(client, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![("testparam".to_owned(), "testparamvalue".to_owned())],
            credentials: None,
            sign: false,
        };

        let data = client
            .send(request)
            .await
            .unwrap()
            .into_body_str()
            .await
            .unwrap();

        assert_eq!(data, "Test Response".to_owned());
    }

    #[tokio::test]
    async fn client_respects_request_credentials_no_signature_test() {
        let client = Client::builder().build(
            MockConnector::new()
                .header("X-MBX-APIKEY", Some("api-key"))
                .response(200, "Test Response"),
        );
        let client = BinanceHttpClient::new(client, "https://base-url.com");

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

        let data = client
            .send(request)
            .await
            .unwrap()
            .into_body_str()
            .await
            .unwrap();

        assert_eq!(data, "Test Response".to_owned());
    }

    #[tokio::test]
    async fn client_respects_request_credentials_with_signature_test() {
        let client = Client::builder().build(
            MockConnector::new()
                .header("X-MBX-APIKEY", Some("api-key"))
                .param("timestamp", None)
                .param("signature", None)
                .response(200, "Test Response"),
        );
        let client = BinanceHttpClient::new(client, "https://base-url.com");

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

        let data = client
            .send(request)
            .await
            .unwrap()
            .into_body_str()
            .await
            .unwrap();

        assert_eq!(data, "Test Response".to_owned());
    }

    #[tokio::test]
    async fn client_handles_not_found_error_test() {
        let client = Client::builder().build(MockConnector::new().response(404, ""));
        let client = BinanceHttpClient::new(client, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client
            .send(request)
            .await
            .unwrap()
            .into_body_str()
            .await
            .unwrap_err();

        match err {
            Error::Client(ClientError::Raw(err)) => assert_eq!(err.status_code, 404),
            _ => panic!("Invalid error"),
        }
    }

    #[tokio::test]
    async fn client_handles_structured_bad_request_test() {
        let client = Client::builder().build(
            MockConnector::new()
                .response(400, "{ \"code\": -1102, \"msg\": \"Mandatory parameter 'symbol' was not sent, was empty/null, or malformed.\" }"),
        );
        let client = BinanceHttpClient::new(client, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client
            .send(request)
            .await
            .unwrap()
            .into_body_str()
            .await
            .unwrap_err();

        match err {
            Error::Client(ClientError::Structured(err)) => assert_eq!(err.status_code, 400),
            _ => panic!("Invalid error"),
        }
    }

    #[tokio::test]
    async fn client_handles_raw_bad_request_test() {
        let client = Client::builder().build(MockConnector::new().response(400, "Error"));
        let client = BinanceHttpClient::new(client, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client
            .send(request)
            .await
            .unwrap()
            .into_body_str()
            .await
            .unwrap_err();

        match err {
            Error::Client(ClientError::Raw(err)) => assert_eq!(err.status_code, 400),
            _ => panic!("Invalid error"),
        }
    }

    #[tokio::test]
    async fn client_handles_server_error_test() {
        let client = Client::builder().build(MockConnector::new().response(500, "Error"));
        let client = BinanceHttpClient::new(client, "https://base-url.com");

        let request = Request {
            method: Method::Get,
            path: "/path".to_owned(),
            params: vec![],
            credentials: None,
            sign: false,
        };

        let err = client
            .send(request)
            .await
            .unwrap()
            .into_body_str()
            .await
            .unwrap_err();

        match err {
            Error::Server(err) => assert_eq!(err.status_code, 500),
            _ => panic!("Invalid error"),
        }
    }

    #[derive(Clone)]
    struct MockConnector {
        base_url: Option<String>,
        path: Option<String>,
        method: Option<String>,
        params: Vec<(String, Option<String>)>,
        headers: HashMap<String, Option<String>>,
        response_code: Option<u16>,
        response_body: Option<String>,
    }

    impl MockConnector {
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

    struct MockResponse {
        response_code: u16,
        response_body: String,
        path: Option<String>,
        params: Vec<(String, Option<String>)>,
        method: Option<String>,
        headers: HashMap<String, Option<String>>,
        ready: bool,
        pos: usize,
        waker: Option<Waker>,
        is_match: bool,
    }
    impl MockResponse {
        pub fn new(
            is_match: bool,
            path: Option<String>,
            params: Vec<(String, Option<String>)>,
            method: Option<String>,
            headers: HashMap<String, Option<String>>,
            code: u16,
            body: &str,
        ) -> Self {
            Self {
                response_code: code,
                response_body: body.to_owned(),
                path,
                params,
                method,
                headers,
                ready: false,
                waker: None,
                pos: 0,
                is_match,
            }
        }

        fn has_header(msg: &str, name: &str, value: &Option<String>) -> bool {
            let n = msg.find("\r\n\r\n").unwrap_or(msg.len());

            let expected_header = format!(
                "{}: {}",
                name.to_lowercase(),
                value.clone().unwrap_or_default()
            );
            msg[..n].contains(&expected_header)
        }
    }

    impl tokio::io::AsyncRead for MockResponse {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<Result<(), IoError>> {
            if !self.ready {
                self.waker = Some(cx.waker().clone());
                Poll::Pending
            } else {
                let data = if self.is_match {
                    format!(
                        "HTTP/1.1 {}\r\n\
                        \r\n\
                        {}",
                        self.response_code, self.response_body
                    )
                } else {
                    format!(
                        "HTTP/1.1 {}\r\n\
                        \r\n\
                        {}",
                        404, ""
                    )
                };
                let data = data.as_bytes();

                let n = std::cmp::min(buf.remaining(), data.len() - self.pos);
                let read_until = self.pos + n;
                buf.put_slice(&data[self.pos..read_until]);
                self.pos = read_until;
                self.waker = Some(cx.waker().clone());
                Poll::Ready(Ok(()))
            }
        }
    }

    impl tokio::io::AsyncWrite for MockResponse {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, IoError>> {
            let msg = str::from_utf8(buf).unwrap();
            let first_line_end = msg.find("\r\n").unwrap_or(msg.len());
            let mut first_line = msg[..first_line_end].split(" ");
            let method = first_line.next().unwrap();
            let path_and_query = first_line.next().unwrap();

            self.is_match = self.is_match
                // Assert request path.
                && self
                    .path
                    .as_ref()
                    .map(|p| path_and_query.starts_with(p))
                    .unwrap_or(true)
                // Assert request query parameters, if they exist.
                && self.params.iter().all(|(k, v)| {
                    let expected_query_param = format!("{}{}", k, v.as_ref().map(|v| format!("={}", v)).unwrap_or_default());
                    path_and_query.contains(&expected_query_param)
                })
                && self
                    .method
                    .as_ref()
                    .map(|m| method == m)
                    .unwrap_or(true)
                && self
                    .headers
                    .iter()
                    .all(|(k, v)| Self::has_header(msg, k, v));

            let Self { ready, waker, .. } = self.get_mut();
            *ready = true;
            waker.take().map(|w| w.wake());
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), IoError>> {
            Poll::Ready(Ok(()))
        }

        fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), IoError>> {
            Poll::Ready(Ok(()))
        }
    }

    impl hyper::client::connect::Connection for MockResponse {
        fn connected(&self) -> Connected {
            Connected::new()
        }
    }

    impl tower::Service<Uri> for MockConnector {
        type Response = MockResponse;
        type Error = http::Error;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: Uri) -> Self::Future {
            let is_match = self
                .base_url
                .as_ref()
                .map(|u| u == &format!("{}://{}", req.scheme().unwrap(), req.host().unwrap()))
                .unwrap_or(true);

            // Create the HTTP response
            let resp = MockResponse::new(
                is_match,
                self.path.clone(),
                self.params.clone(),
                self.method.clone(),
                self.headers.clone(),
                self.response_code.unwrap_or(200),
                match &self.response_body {
                    Some(body) => body.as_str(),
                    None => "",
                },
            );

            // create a response in a future.
            let fut = async { Ok(resp) };

            // Return the response as an immediate future
            Box::pin(fut)
        }
    }
}
