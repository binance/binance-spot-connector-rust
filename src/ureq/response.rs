use crate::http::error::{BinanceApiError, ClientError, HttpError};
use crate::ureq::Error;
use std::collections::HashMap;
use ureq::Response as UreqResponse;

/// REST Response
pub struct Response {
    inner_response: UreqResponse,
}

impl Response {
    pub fn new(inner_response: UreqResponse) -> Self {
        Self { inner_response }
    }

    /// Fetch the data received from the API.
    pub fn into_body_str(self) -> Result<String, Box<Error>> {
        let status = self.inner_response.status();
        if 400 <= status {
            let headers: HashMap<String, String> = self.inner_response.headers_names().iter().fold(
                HashMap::new(),
                |mut headers, k| {
                    if let Some(header) = self.inner_response.header(k) {
                        headers
                            .entry(k.as_str().to_owned())
                            .or_insert_with(|| header.to_owned());
                    }

                    headers
                },
            );

            let content = self
                .inner_response
                .into_string()
                .expect("Response failed UTF-8 encoding.");
            if 500 <= status {
                Err(Box::new(Error::Server(HttpError::new(
                    status, content, headers,
                ))))
            } else {
                let client_error = match serde_json::from_str::<BinanceApiError>(&content) {
                    Ok(err) => ClientError::Structured(HttpError::new(status, err, headers)),
                    Err(_) => ClientError::Raw(HttpError::new(status, content, headers)),
                };

                Err(Box::new(Error::Client(client_error)))
            }
        } else {
            Ok(self
                .inner_response
                .into_string()
                .expect("Response failed UTF-8 encoding."))
        }
    }
}

impl From<UreqResponse> for Response {
    fn from(response: UreqResponse) -> Response {
        Response {
            inner_response: response,
        }
    }
}

impl From<Response> for UreqResponse {
    fn from(response: Response) -> UreqResponse {
        response.inner_response
    }
}
