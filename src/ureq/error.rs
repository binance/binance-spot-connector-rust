use crate::http::error::{ClientError, HttpError as BinanceHttpError};
use http::{uri::InvalidUri, Error as HttpError};
use ureq::Error as UreqError;

/// Communication error with the server.
#[derive(Debug)]
pub enum Error {
    /// 4XX error from the server.
    Client(ClientError),
    /// 5XX error from the server.
    Server(BinanceHttpError<String>),
    /// The format of the API secret is invalid.
    InvalidApiSecret,
    Parse(HttpError),
    Send(UreqError),
}

impl From<InvalidUri> for Box<Error> {
    fn from(err: InvalidUri) -> Box<Error> {
        Box::new(Error::Parse(err.into()))
    }
}
