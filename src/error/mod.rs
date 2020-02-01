
use serde_json;
use reqwest;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ReqError(reqwest::Error),
    JsonError(serde_json::Error),

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonError(ref cause) => write!(f, "serde_json error: {}", cause),
            Error::ReqError(ref cause) => write!(f, "reqwest error : {}",  cause),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(cause: reqwest::Error) -> Error {
        Error::ReqError(cause)
    }
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Error {
        Error::JsonError(cause)
    }
}
