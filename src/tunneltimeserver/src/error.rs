use hyper;
use serde_json;
use std::io;

#[derive(Debug)]
pub enum Error {
    SerdeJSON(serde_json::Error),
    IO(io::Error),
    HyperURI(hyper::error::UriError),
    Hyper(hyper::Error),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJSON(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::Hyper(error)
    }
}

impl From<hyper::error::UriError> for Error {
    fn from(error: hyper::error::UriError) -> Self {
        Error::HyperURI(error)
    }
}
