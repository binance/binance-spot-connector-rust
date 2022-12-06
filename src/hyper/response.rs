use crate::http::error::{BinanceApiError, ClientError, HttpError};
use crate::hyper::Error;
use hyper::Body;
use std::collections::HashMap;

/// REST Response
#[derive(Debug)]
pub struct Response {
    inner_response: hyper::Response<Body>,
}

impl Response {
    pub async fn into_body_str(self) -> Result<String, Error> {
        let status = self.inner_response.status().as_u16();
        if 400 <= status {
            let headers: HashMap<String, String> =
                self.inner_response
                    .headers()
                    .iter()
                    .fold(HashMap::new(), |mut headers, (k, v)| {
                        headers.entry(k.as_str().to_owned()).or_insert_with(|| {
                            // Assume all Binance response headers can convert to String.
                            v.to_str()
                                .expect("Failed to convert response header value to string")
                                .to_owned()
                        });
                        headers
                    });

            let content = hyper_body_to_string(self.inner_response.into_body()).await?;
            if 500 <= status {
                Err(Error::Server(HttpError::new(status, content, headers)))
            } else {
                let client_error = match serde_json::from_str::<BinanceApiError>(&content) {
                    Ok(err) => ClientError::Structured(HttpError::new(status, err, headers)),
                    Err(_) => ClientError::Raw(HttpError::new(status, content, headers)),
                };

                Err(Error::Client(client_error))
            }
        } else {
            Ok(hyper_body_to_string(self.inner_response.into_body()).await?)
        }
    }
}

impl From<hyper::Response<Body>> for Response {
    fn from(response: hyper::Response<Body>) -> Response {
        Response {
            inner_response: response,
        }
    }
}

impl From<Response> for hyper::Response<Body> {
    fn from(response: Response) -> hyper::Response<Body> {
        response.inner_response
    }
}

async fn hyper_body_to_string(body: Body) -> Result<String, Error> {
    // Assume all Binance responses are of a reasonable size.
    let body = hyper::body::to_bytes(body)
        .await
        .expect("Failed to collect response body.");

    // Assume all Binance responses are in UTF-8.
    let content = String::from_utf8(body.to_vec()).expect("Response failed UTF-8 encoding.");

    Ok(content)
}
